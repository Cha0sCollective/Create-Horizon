package dev.ch4os.reference;

import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import com.simibubi.create.AllBlocks;
import com.simibubi.create.api.stress.BlockStressValues;
import com.simibubi.create.content.kinetics.base.KineticBlockEntity;
import com.simibubi.create.content.kinetics.crank.HandCrankBlock;
import com.simibubi.create.content.kinetics.crank.HandCrankBlockEntity;
import com.simibubi.create.content.kinetics.millstone.MillstoneBlockEntity;
import com.simibubi.create.content.kinetics.motor.CreativeMotorBlock;
import com.simibubi.create.content.kinetics.motor.CreativeMotorBlockEntity;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.UUID;
import java.nio.charset.StandardCharsets;
import com.simibubi.create.foundation.blockEntity.behaviour.ValueSettingsBehaviour.ValueSettings;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.gametest.framework.GameTest;
import net.minecraft.gametest.framework.GameTestHelper;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.gametest.GameTestHolder;
import net.neoforged.neoforge.gametest.PrefixGameTestTemplate;

/** Observes real server block entities; contains no reimplementation of their rules. */
@GameTestHolder("create_horizon_reference")
@PrefixGameTestTemplate(false)
public class PortReferenceTests {
    private static final Gson JSON = new Gson();
    private static final BlockPos MOTOR = new BlockPos(2, 2, 2);
    private static final BlockPos DRIVE = MOTOR.above();
    private static final BlockPos LOAD = DRIVE.above();

    static BlockState axial(BlockState state) {
        return state.setValue(BlockStateProperties.AXIS, Direction.Axis.Y);
    }

    private static void motor(GameTestHelper helper) {
        helper.setBlock(MOTOR, AllBlocks.CREATIVE_MOTOR.getDefaultState()
                .setValue(CreativeMotorBlock.FACING, Direction.UP));
    }

    static KineticBlockEntity kinetic(GameTestHelper helper, BlockPos pos) {
        return (KineticBlockEntity) helper.getBlockEntity(pos);
    }

    private static void speed(GameTestHelper helper, int rpm) {
        ((CreativeMotorBlockEntity) helper.getBlockEntity(MOTOR)).generatedSpeed.setValue(rpm);
    }

    private static void sample(GameTestHelper helper, String fixture, BlockPos... positions) {
        sample(helper, fixture, Map.of(), positions);
    }

    static void sample(GameTestHelper helper, String fixture, Map<String, Object> context, BlockPos... positions) {
        boolean sourceObservation = fixture.startsWith("arbitration_") || fixture.startsWith("horizontal_");
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("schema", sourceObservation ? 3 : 2);
        row.put("fixture", fixture);
        row.put("tick", helper.getTick());
        row.put("phase", "gametest_scheduled_callback");
        row.put("dimension", helper.getLevel().dimension().location().toString());
        row.put("world_seed", helper.getLevel().getSeed());
        row.put("context", context);
        var machines = new ArrayList<Map<String, Object>>();
        for (BlockPos pos : positions) {
            var value = new LinkedHashMap<String, Object>();
            value.put("position", new int[] {pos.getX(), pos.getY(), pos.getZ()});
            BlockState state = helper.getBlockState(pos);
            value.put("block", BuiltInRegistries.BLOCK.getKey(state.getBlock()).toString());
            var properties = new java.util.TreeMap<String, String>();
            state.getValues().forEach((key, val) -> properties.put(key.getName(), val.toString()));
            value.put("properties", properties);
            var entity = helper.getLevel().getBlockEntity(helper.absolutePos(pos));
            if (entity instanceof KineticBlockEntity be) {
                value.put("theoretical_rpm", be.getTheoreticalSpeed());
                value.put("effective_rpm", be.getSpeed());
                if (sourceObservation) {
                    value.put("generated_rpm", be.getGeneratedSpeed());
                    value.put("flicker", be.getFlickerScore());
                    value.put("source_parent", be.source == null ? new int[] {} : relative(helper, be.source, positions));
                    var group = new ArrayList<int[]>();
                    for (BlockPos member : positions) {
                        var other = helper.getLevel().getBlockEntity(helper.absolutePos(member));
                        if (be.hasNetwork() && other instanceof KineticBlockEntity peer && be.network.equals(peer.network))
                            group.add(new int[] {member.getX(), member.getY(), member.getZ()});
                    }
                    value.put("network_members", group);
                    if (be instanceof CreativeMotorBlockEntity motor)
                        value.put("setting", motor.generatedSpeed.getValue());
                }
                CompoundTag tag = be.saveWithoutMetadata(helper.getLevel().registryAccess());
                value.put("network", Map.of("present", be.hasNetwork(),
                        "stress_su", tag.getCompound("Network").getFloat("Stress"),
                        "capacity_su", tag.getCompound("Network").getFloat("Capacity")));
                if (be instanceof MillstoneBlockEntity mill) {
                    value.put("remaining_work", mill.timer);
                    value.put("output_slot_limit", mill.outputInv.getSlotLimit(0));
                    var slots = new ArrayList<Map<String, Object>>();
                    slots.add(stack("input", 0, mill.inputInv.getStackInSlot(0)));
                    for (int slot = 0; slot < mill.outputInv.getSlots(); slot++)
                        slots.add(stack("output", slot, mill.outputInv.getStackInSlot(slot)));
                    value.put("slots", slots);
                }
            }
            machines.add(value);
        }
        row.put("machines", machines);
        System.out.println("CREATE_HORIZON_TRACE " + JSON.toJson(row));
    }

    private static int[] relative(GameTestHelper helper, BlockPos world, BlockPos[] observed) {
        for (BlockPos pos : observed)
            if (helper.absolutePos(pos).equals(world))
                return new int[] {pos.getX(), pos.getY(), pos.getZ()};
        throw new IllegalStateException("Source parent is outside the observed fixture: " + world);
    }

    private static Map<String, Object> stack(String owner, int slot, ItemStack stack) {
        return Map.of("owner", owner, "slot", slot, "item",
                BuiltInRegistries.ITEM.getKey(stack.getItem()).toString(), "count", stack.getCount());
    }

    @GameTest(template = "empty", timeoutTicks = 100)
    public static void directStopReverse(GameTestHelper helper) {
        motor(helper);
        helper.setBlock(DRIVE, axial(AllBlocks.SHAFT.getDefaultState()));
        helper.setBlock(LOAD, axial(AllBlocks.SHAFT.getDefaultState()));
        for (int tick = 1; tick <= 70; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                sample(helper, "direct_stop_reverse", MOTOR, DRIVE, LOAD);
                if (at == 10) {
                    helper.assertTrue(kinetic(helper, LOAD).getSpeed() == 16, "Direct drive must settle to +16 RPM");
                    speed(helper, -16);
                }
                if (at == 30) {
                    helper.assertTrue(kinetic(helper, LOAD).getSpeed() == -16, "Reversal must reach the load");
                    helper.setBlock(DRIVE, Blocks.AIR);
                }
                if (at == 70) {
                    helper.assertTrue(kinetic(helper, LOAD).getSpeed() == 0, "Disconnected load must stop");
                    helper.succeed();
                }
            });
        }
    }

    private static void gears(GameTestHelper helper, String fixture, boolean largeDriver, boolean largeLoad) {
        motor(helper);
        helper.setBlock(DRIVE, axial((largeDriver ? AllBlocks.LARGE_COGWHEEL : AllBlocks.COGWHEEL).getDefaultState()));
        BlockPos target = DRIVE.offset(1, 0, largeDriver || largeLoad ? 1 : 0);
        helper.setBlock(target, axial((largeLoad ? AllBlocks.LARGE_COGWHEEL : AllBlocks.COGWHEEL).getDefaultState()));
        for (int tick = 1; tick <= 20; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                sample(helper, fixture, MOTOR, DRIVE, target);
                if (at == 20) {
                    float expected = largeDriver ? -32 : largeLoad ? -8 : -16;
                    helper.assertTrue(kinetic(helper, target).getSpeed() == expected, "Unexpected settled cog ratio");
                    helper.succeed();
                }
            });
        }
    }

    @GameTest(template = "empty", timeoutTicks = 60)
    public static void smallToSmall(GameTestHelper helper) { gears(helper, "small_to_small", false, false); }

    @GameTest(template = "empty", timeoutTicks = 60)
    public static void largeToSmall(GameTestHelper helper) { gears(helper, "large_to_small", true, false); }

    @GameTest(template = "empty", timeoutTicks = 60)
    public static void smallToLarge(GameTestHelper helper) { gears(helper, "small_to_large", false, true); }

    private static void milling(GameTestHelper helper, int rpm) {
        motor(helper);
        helper.setBlock(DRIVE, axial(AllBlocks.SHAFT.getDefaultState()));
        helper.setBlock(LOAD, AllBlocks.MILLSTONE.getDefaultState());
        ItemEntity[] dropped = {null};
        for (int tick = 1; tick <= 300; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                sample(helper, "milling_" + rpm, MOTOR, DRIVE, LOAD);
                if (at == 10) {
                    speed(helper, rpm);
                    BlockPos world = helper.absolutePos(LOAD);
                    ItemEntity item = new ItemEntity(helper.getLevel(), world.getX() + .5,
                            world.getY() + 1.2, world.getZ() + .5, new ItemStack(Items.COBBLESTONE));
                    item.setDeltaMovement(0, 0, 0);
                    helper.getLevel().addFreshEntity(item);
                    dropped[0] = item;
                }
                if (at == 300) {
                    MillstoneBlockEntity mill = (MillstoneBlockEntity) helper.getBlockEntity(LOAD);
                    helper.assertTrue(dropped[0] != null && !dropped[0].isAlive(), "Dropped input entity must be consumed");
                    helper.assertTrue(mill.inputInv.getStackInSlot(0).isEmpty(), "Input must be consumed");
                    int gravel = 0;
                    for (int slot = 0; slot < mill.outputInv.getSlots(); slot++) {
                        ItemStack stack = mill.outputInv.getStackInSlot(slot);
                        helper.assertTrue(stack.isEmpty() || stack.is(Items.GRAVEL), "Unexpected output item");
                        gravel += stack.getCount();
                    }
                    helper.assertTrue(gravel == 1, "One cobblestone must yield exactly one gravel");
                    helper.succeed();
                }
            });
        }
    }

    @GameTest(template = "empty", timeoutTicks = 340)
    public static void milling16(GameTestHelper helper) { milling(helper, 16); }

    @GameTest(template = "empty", timeoutTicks = 340)
    public static void milling256(GameTestHelper helper) { milling(helper, 256); }

    private static void assertNetwork(GameTestHelper helper, BlockPos position, float stress, float capacity, boolean running) {
        KineticBlockEntity machine = kinetic(helper, position);
        CompoundTag network = machine.saveWithoutMetadata(helper.getLevel().registryAccess()).getCompound("Network");
        helper.assertTrue(network.getFloat("Stress") == stress, "Unexpected finite-network stress");
        helper.assertTrue(network.getFloat("Capacity") == capacity, "Unexpected finite-network capacity");
        helper.assertTrue(Math.abs(machine.getTheoreticalSpeed()) == 32, "Theoretical crank speed must remain 32 RPM");
        helper.assertTrue(running ? Math.abs(machine.getSpeed()) == 32 : machine.getSpeed() == 0,
                "Unexpected effective speed at stress boundary");
    }

    @GameTest(template = "empty", timeoutTicks = 160)
    public static void finiteStress(GameTestHelper helper) {
        BlockPos cog = DRIVE.above();
        BlockPos second = cog.offset(1, 0, 0);
        BlockPos third = cog.offset(0, 0, 1);
        helper.setBlock(DRIVE, axial(AllBlocks.SHAFT.getDefaultState()));
        helper.setBlock(MOTOR, AllBlocks.HAND_CRANK.getDefaultState().setValue(HandCrankBlock.FACING, Direction.DOWN));
        for (BlockPos pos : new BlockPos[] {cog, second, third})
            helper.setBlock(pos, axial(AllBlocks.COGWHEEL.getDefaultState()));
        helper.setBlock(cog.above(), AllBlocks.MILLSTONE.getDefaultState());
        double sourceCapacity = BlockStressValues.getCapacity(AllBlocks.HAND_CRANK.get());
        double millImpact = BlockStressValues.getImpact(AllBlocks.MILLSTONE.get());
        helper.assertTrue(sourceCapacity == 8 && millImpact == 4, "Fixture requires declared default stress values");
        for (int tick = 1; tick <= 120; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                sample(helper, "finite_stress", Map.of("capacity_per_rpm", sourceCapacity, "millstone_impact_per_rpm", millImpact),
                        MOTOR, DRIVE, cog, second, third, cog.above(), second.above(), third.above());
                if (at % 5 == 0 && at <= 95)
                    ((HandCrankBlockEntity) helper.getBlockEntity(MOTOR)).turn(false);
                if (at == 20) assertNetwork(helper, cog.above(), 128, 256, true);
                if (at == 30) helper.setBlock(second.above(), AllBlocks.MILLSTONE.getDefaultState());
                if (at == 50) {
                    assertNetwork(helper, cog.above(), 256, 256, true);
                    helper.setBlock(third.above(), AllBlocks.MILLSTONE.getDefaultState());
                }
                if (at == 70) {
                    assertNetwork(helper, cog.above(), 384, 256, false);
                    helper.setBlock(third.above(), Blocks.AIR);
                }
                if (at == 90) assertNetwork(helper, cog.above(), 256, 256, true);
                if (at == 120) {
                    helper.assertTrue(kinetic(helper, cog.above()).getSpeed() == 0, "Expired crank must stop loads");
                    helper.succeed();
                }
            });
        }
    }

    private static ItemEntity dropInput(GameTestHelper helper) {
        BlockPos world = helper.absolutePos(LOAD);
        ItemEntity item = new ItemEntity(helper.getLevel(), world.getX() + .5, world.getY() + 1.2,
                world.getZ() + .5, new ItemStack(Items.COBBLESTONE));
        item.setDeltaMovement(0, 0, 0);
        helper.getLevel().addFreshEntity(item);
        return item;
    }

    private static int outputCount(MillstoneBlockEntity mill) {
        int count = 0;
        for (int slot = 0; slot < mill.outputInv.getSlots(); slot++) count += mill.outputInv.getStackInSlot(slot).getCount();
        return count;
    }

    @GameTest(template = "empty", timeoutTicks = 380)
    public static void millingInterruptReverse(GameTestHelper helper) {
        motor(helper);
        helper.setBlock(DRIVE, axial(AllBlocks.SHAFT.getDefaultState()));
        helper.setBlock(LOAD, AllBlocks.MILLSTONE.getDefaultState());
        ItemEntity[] dropped = {null};
        int[] pausedWork = {-1};
        for (int tick = 1; tick <= 340; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                MillstoneBlockEntity mill = (MillstoneBlockEntity) helper.getBlockEntity(LOAD);
                sample(helper, "milling_interrupt_reverse", Map.of("paused_work", pausedWork[0]), MOTOR, DRIVE, LOAD);
                if (at == 10) dropped[0] = dropInput(helper);
                if (at == 40) {
                    helper.assertTrue(mill.timer > 0 && mill.inputInv.getStackInSlot(0).getCount() == 1, "Recipe must be in flight");
                    pausedWork[0] = mill.timer;
                    helper.setBlock(DRIVE, Blocks.AIR);
                }
                if (at == 80) {
                    helper.assertTrue(mill.timer == pausedWork[0] && mill.getSpeed() == 0 && outputCount(mill) == 0,
                            "Disconnected milling must preserve work and input");
                    helper.setBlock(DRIVE, axial(AllBlocks.SHAFT.getDefaultState()));
                    speed(helper, -16);
                }
                if (at == 100) helper.assertTrue(mill.getSpeed() == -16, "Reconnected mill must reverse");
                if (at == 340) {
                    helper.assertTrue(!dropped[0].isAlive() && mill.inputInv.getStackInSlot(0).isEmpty(), "Input must be consumed once");
                    helper.assertTrue(outputCount(mill) == 1 && mill.outputInv.getStackInSlot(0).is(Items.GRAVEL), "Resumed recipe must yield one gravel");
                    helper.succeed();
                }
            });
        }
    }

    private static void inventoryEdge(GameTestHelper helper, String fixture) {
        motor(helper);
        helper.setBlock(DRIVE, axial(AllBlocks.SHAFT.getDefaultState()));
        helper.setBlock(LOAD, AllBlocks.MILLSTONE.getDefaultState());
        ItemEntity[] dropped = {null};
        ItemStack[] retained = {ItemStack.EMPTY, fixture.equals("milling_input_changed") ? new ItemStack(Items.BARRIER) : ItemStack.EMPTY};
        for (int tick = 1; tick <= 100; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                MillstoneBlockEntity mill = (MillstoneBlockEntity) helper.getBlockEntity(LOAD);
                sample(helper, fixture, Map.of("external_cobblestone", retained[0].is(Items.COBBLESTONE) ? retained[0].getCount() : 0,
                        "external_barrier", retained[1].getCount(), "extracted_gravel", retained[0].is(Items.GRAVEL) ? retained[0].getCount() : 0), MOTOR, DRIVE, LOAD);
                if (at == 10) {
                    speed(helper, 256);
                    if (fixture.equals("milling_full_output")) {
                        for (int slot = 0; slot < 9; slot++) mill.outputInv.setStackInSlot(slot, new ItemStack(Items.GRAVEL, 64));
                    }
                    if (fixture.equals("milling_one_full_slot")) mill.outputInv.setStackInSlot(0, new ItemStack(Items.GRAVEL, 64));
                    dropped[0] = dropInput(helper);
                }
                if (at == 20 && (fixture.equals("milling_input_changed") || fixture.equals("milling_input_removed"))) {
                    helper.assertTrue(mill.timer > 0 && mill.inputInv.getStackInSlot(0).getCount() == 1, "Mutation must interrupt an active recipe");
                    retained[0] = mill.inputInv.extractItem(0, 1, false);
                    if (fixture.equals("milling_input_changed")) {
                        mill.inputInv.setStackInSlot(0, retained[1].copy());
                        retained[1].shrink(1);
                    }
                }
                if (at == 40 && fixture.equals("milling_full_output")) {
                    helper.assertTrue(mill.outputInv.getSlotLimit(0) == 99, "Record the pinned NeoForge absolute slot limit");
                    helper.assertTrue(mill.timer <= 0 && mill.inputInv.getStackInSlot(0).isEmpty() && outputCount(mill) == 576,
                            "Pinned reference consumes input while all normal output stacks are full; record this divergence candidate");
                }
                if (at == 100) {
                    helper.assertTrue(dropped[0] != null && !dropped[0].isAlive(), "Dropped item must have been accepted once");
                    if (fixture.equals("milling_full_output")) {
                        helper.assertTrue(mill.inputInv.getStackInSlot(0).isEmpty() && outputCount(mill) == 576,
                                "Record the lost uninsertable result in the pinned reference, not a native conservation pass");
                    } else if (fixture.equals("milling_one_full_slot")) {
                        helper.assertTrue(mill.inputInv.getStackInSlot(0).isEmpty() && outputCount(mill) == 65,
                                "One 64-item stack must leave room in the other output slots");
                    } else {
                        helper.assertTrue(outputCount(mill) == 0 && retained[0].is(Items.COBBLESTONE) && retained[0].getCount() == 1,
                                "Changed input must not produce or consume the removed cobblestone");
                        ItemStack input = mill.inputInv.getStackInSlot(0);
                        helper.assertTrue(fixture.equals("milling_input_changed") ? input.is(Items.BARRIER) && input.getCount() == 1 : input.isEmpty(),
                                "Completion must revalidate the current input");
                    }
                    helper.succeed();
                }
            });
        }
    }

    @GameTest(template = "empty", timeoutTicks = 140)
    public static void millingFullOutput(GameTestHelper helper) { inventoryEdge(helper, "milling_full_output"); }

    @GameTest(template = "empty", timeoutTicks = 140)
    public static void millingOneFullSlot(GameTestHelper helper) { inventoryEdge(helper, "milling_one_full_slot"); }

    @GameTest(template = "empty", timeoutTicks = 140)
    public static void millingInputChanged(GameTestHelper helper) { inventoryEdge(helper, "milling_input_changed"); }

    @GameTest(template = "empty", timeoutTicks = 140)
    public static void millingInputRemoved(GameTestHelper helper) { inventoryEdge(helper, "milling_input_removed"); }

    private static FakePlayer actor(GameTestHelper helper, String fixture) {
        FakePlayer actor = FakePlayerFactory.get(helper.getLevel(), new GameProfile(
                UUID.nameUUIDFromBytes(fixture.getBytes(StandardCharsets.UTF_8)), "PortReference"));
        actor.getInventory().clearContent();
        BlockPos world = helper.absolutePos(new BlockPos(2, 1, 6));
        actor.setPos(world.getX() + .5, world.getY(), world.getZ() + .5);
        actor.setYRot(0);
        return actor;
    }

    private static int held(FakePlayer actor, net.minecraft.world.item.Item item) {
        int count = 0;
        for (int slot = 0; slot < actor.getInventory().getContainerSize(); slot++) {
            ItemStack stack = actor.getInventory().getItem(slot);
            if (stack.is(item)) count += stack.getCount();
        }
        return count;
    }

    private static int ground(GameTestHelper helper, net.minecraft.world.item.Item item) {
        AABB bounds = new AABB(Vec3.atLowerCornerOf(helper.absolutePos(BlockPos.ZERO)),
                Vec3.atLowerCornerOf(helper.absolutePos(new BlockPos(16, 8, 16))));
        return helper.getLevel().getEntitiesOfClass(ItemEntity.class, bounds).stream()
                .filter(entity -> entity.isAlive() && entity.getItem().is(item))
                .mapToInt(entity -> entity.getItem().getCount()).sum();
    }

    private static void retrieve(GameTestHelper helper, FakePlayer actor) {
        BlockPos world = helper.absolutePos(LOAD);
        // Real server block handler; this intentionally does not claim network/reach validation.
        helper.getBlockState(LOAD).useItemOn(ItemStack.EMPTY, helper.getLevel(), actor, InteractionHand.OFF_HAND,
                new BlockHitResult(Vec3.atCenterOf(world), Direction.SOUTH, world, false));
    }

    private static void retrieval(GameTestHelper helper, boolean overflow) {
        String fixture = overflow ? "retrieval_overflow" : "retrieval_empty_hand";
        helper.setBlock(LOAD, AllBlocks.MILLSTONE.getDefaultState());
        // Keep overflow entities in this fixture and away from the mill's intake.
        for (int x = 0; x < 16; x++) for (int z = 0; z < 16; z++)
            helper.setBlock(new BlockPos(x, 0, z), Blocks.STONE);
        FakePlayer actor = actor(helper, fixture);
        if (overflow) for (int slot = 0; slot < 36; slot++)
            actor.getInventory().setItem(slot, new ItemStack(Items.IRON_INGOT, 64));
        for (int tick = 1; tick <= 40; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                MillstoneBlockEntity mill = (MillstoneBlockEntity) helper.getBlockEntity(LOAD);
                sample(helper, fixture, Map.of("player_gravel", held(actor, Items.GRAVEL),
                        "player_cobblestone", held(actor, Items.COBBLESTONE), "player_iron", held(actor, Items.IRON_INGOT),
                        "ground_gravel", ground(helper, Items.GRAVEL), "ground_cobblestone", ground(helper, Items.COBBLESTONE)), LOAD);
                if (at == 10) {
                    mill.inputInv.setStackInSlot(0, new ItemStack(Items.COBBLESTONE, 3));
                    mill.outputInv.setStackInSlot(0, new ItemStack(Items.GRAVEL, 5));
                    retrieve(helper, actor);
                    helper.assertTrue(mill.inputInv.getStackInSlot(0).getCount() == 3 && outputCount(mill) == 0,
                            "First empty-hand use retrieves output and preserves input");
                }
                if (at == 20) {
                    retrieve(helper, actor);
                    helper.assertTrue(mill.inputInv.getStackInSlot(0).isEmpty(), "Second empty-hand use retrieves input fallback");
                }
                if (at == 40) {
                    helper.assertTrue(held(actor, Items.GRAVEL) + ground(helper, Items.GRAVEL) == 5
                            && held(actor, Items.COBBLESTONE) + ground(helper, Items.COBBLESTONE) == 3,
                            "Retrieval must conserve all fixture items");
                    helper.assertTrue(held(actor, Items.IRON_INGOT) == (overflow ? 2304 : 0)
                            && held(actor, Items.GRAVEL) == (overflow ? 0 : 5)
                            && held(actor, Items.COBBLESTONE) == (overflow ? 0 : 3), "Unexpected player overflow accounting");
                    helper.succeed();
                }
            });
        }
    }

    @GameTest(template = "empty", timeoutTicks = 80)
    public static void retrievalEmptyHand(GameTestHelper helper) { retrieval(helper, false); }

    @GameTest(template = "empty", timeoutTicks = 80)
    public static void retrievalOverflow(GameTestHelper helper) { retrieval(helper, true); }

    @GameTest(template = "empty", timeoutTicks = 100)
    public static void motorValueSettings(GameTestHelper helper) {
        motor(helper);
        helper.setBlock(DRIVE, axial(AllBlocks.SHAFT.getDefaultState()));
        FakePlayer actor = actor(helper, "motor_value_settings");
        for (int tick = 1; tick <= 60; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                CreativeMotorBlockEntity motor = (CreativeMotorBlockEntity) helper.getBlockEntity(MOTOR);
                sample(helper, "motor_value_settings", Map.of("setting", motor.generatedSpeed.getValue()), MOTOR, DRIVE);
                if (at == 10) {
                    helper.assertTrue(motor.generatedSpeed.getValue() == 16, "Default setting must be +16");
                    motor.generatedSpeed.setValueSettings(actor, new ValueSettings(1, 0), false);
                }
                if (at == 20) {
                    helper.assertTrue(motor.generatedSpeed.getValue() == 1 && kinetic(helper, DRIVE).getSpeed() == 1,
                            "Positive UI magnitude zero clamps to +1");
                    motor.generatedSpeed.setValueSettings(actor, new ValueSettings(0, 0), false);
                }
                if (at == 30) {
                    helper.assertTrue(motor.generatedSpeed.getValue() == -1 && kinetic(helper, DRIVE).getSpeed() == -1,
                            "Negative UI magnitude zero clamps to -1");
                    motor.generatedSpeed.setValueSettings(actor, new ValueSettings(0, 256), false);
                }
                if (at == 40) {
                    helper.assertTrue(motor.generatedSpeed.getValue() == -256 && kinetic(helper, DRIVE).getSpeed() == -256,
                            "Negative control reaches -256");
                    motor.generatedSpeed.setValueSettings(actor, new ValueSettings(1, 256), false);
                }
                if (at == 60) {
                    helper.assertTrue(motor.generatedSpeed.getValue() == 256 && kinetic(helper, DRIVE).getSpeed() == 256,
                            "Positive control reaches +256");
                    helper.succeed();
                }
            });
        }
    }
}
