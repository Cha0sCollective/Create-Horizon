package dev.ch4os.reference;

import com.google.gson.Gson;
import com.simibubi.create.AllBlocks;
import com.simibubi.create.content.kinetics.base.KineticBlockEntity;
import com.simibubi.create.content.kinetics.millstone.MillstoneBlockEntity;
import com.simibubi.create.content.kinetics.motor.CreativeMotorBlock;
import com.simibubi.create.content.kinetics.motor.CreativeMotorBlockEntity;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.Map;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.gametest.framework.GameTest;
import net.minecraft.gametest.framework.GameTestHelper;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;
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

    private static BlockState axial(BlockState state) {
        return state.setValue(BlockStateProperties.AXIS, Direction.Axis.Y);
    }

    private static void motor(GameTestHelper helper) {
        helper.setBlock(MOTOR, AllBlocks.CREATIVE_MOTOR.getDefaultState()
                .setValue(CreativeMotorBlock.FACING, Direction.UP));
    }

    private static KineticBlockEntity kinetic(GameTestHelper helper, BlockPos pos) {
        return (KineticBlockEntity) helper.getBlockEntity(pos);
    }

    private static void speed(GameTestHelper helper, int rpm) {
        ((CreativeMotorBlockEntity) helper.getBlockEntity(MOTOR)).generatedSpeed.setValue(rpm);
    }

    private static void sample(GameTestHelper helper, String fixture, BlockPos... positions) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("schema", 1);
        row.put("fixture", fixture);
        row.put("tick", helper.getTick());
        row.put("phase", "gametest_scheduled_callback");
        row.put("dimension", helper.getLevel().dimension().location().toString());
        row.put("world_seed", helper.getLevel().getSeed());
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
                CompoundTag tag = be.saveWithoutMetadata(helper.getLevel().registryAccess());
                value.put("network", Map.of("present", be.hasNetwork(),
                        "stress_su", tag.getCompound("Network").getFloat("Stress"),
                        "capacity_su", tag.getCompound("Network").getFloat("Capacity")));
                if (be instanceof MillstoneBlockEntity mill) {
                    value.put("remaining_work", mill.timer);
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
}
