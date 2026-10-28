package dev.ch4os.reference;

import com.simibubi.create.AllBlocks;
import com.simibubi.create.content.kinetics.motor.CreativeMotorBlock;
import com.simibubi.create.content.kinetics.motor.CreativeMotorBlockEntity;
import com.simibubi.create.infrastructure.config.AllConfigs;
import java.util.Map;
import java.util.TreeMap;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.gametest.framework.GameTest;
import net.minecraft.gametest.framework.GameTestHelper;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.neoforged.neoforge.gametest.GameTestHolder;
import net.neoforged.neoforge.gametest.PrefixGameTestTemplate;

/** Six bounded source-arbitration observations, using real public world operations. */
@GameTestHolder("create_horizon_arbitration")
@PrefixGameTestTemplate(false)
public class ArbitrationReferenceTests {
    private static final BlockPos A = new BlockPos(2, 2, 2);
    private static final BlockPos JOIN = A.above();
    private static final BlockPos B = A.above(2);

    private static void floor(GameTestHelper helper) {
        // Retain every dropped item within the fixture for complete item accounting.
        for (int x = 0; x < 16; x++) for (int z = 0; z < 16; z++)
            helper.setBlock(new BlockPos(x, 0, z), Blocks.STONE);
    }

    private static Map<String, Integer> drops(GameTestHelper helper) {
        var result = new TreeMap<String, Integer>();
        AABB bounds = new AABB(Vec3.atLowerCornerOf(helper.absolutePos(BlockPos.ZERO)),
                Vec3.atLowerCornerOf(helper.absolutePos(new BlockPos(16, 8, 16))));
        for (ItemEntity item : helper.getLevel().getEntitiesOfClass(ItemEntity.class, bounds))
            if (item.isAlive()) result.merge(BuiltInRegistries.ITEM.getKey(item.getItem().getItem()).toString(),
                    item.getItem().getCount(), Integer::sum);
        return result;
    }

    private static void request(GameTestHelper helper, BlockPos pos, int generated) {
        var motor = (CreativeMotorBlockEntity) helper.getBlockEntity(pos);
        motor.generatedSpeed.setValue(pos.equals(B) ? -generated : generated);
    }

    private static void node(GameTestHelper helper, BlockPos pos, Block block, float rpm,
            float generated, float capacity, BlockPos parent) {
        helper.assertTrue(helper.getBlockState(pos).is(block), "Wrong surviving block at " + pos);
        if (block == Blocks.AIR) return;
        var be = PortReferenceTests.kinetic(helper, pos);
        helper.assertTrue(be.getTheoreticalSpeed() == rpm && be.getSpeed() == rpm,
                "Wrong theoretical/effective RPM at " + pos);
        helper.assertTrue(be.getGeneratedSpeed() == generated, "Wrong own generated RPM at " + pos);
        helper.assertTrue(parent == null ? be.source == null : helper.absolutePos(parent).equals(be.source),
                "Wrong source parent at " + pos);
        var network = be.saveWithoutMetadata(helper.getLevel().registryAccess()).getCompound("Network");
        helper.assertTrue(be.hasNetwork() == (capacity > 0) && network.getFloat("Capacity") == capacity
                && network.getFloat("Stress") == 0, "Wrong network capacity/stress at " + pos);
    }

    private static void outcome(GameTestHelper helper, String fixture, int at) {
        Block motor = AllBlocks.CREATIVE_MOTOR.get();
        Block shaft = AllBlocks.SHAFT.get();
        Map<String, Integer> expectedDrops = Map.of();
        if (fixture.equals("arbitration_opposite_merge")) {
            node(helper, A, motor, 16, 16, 262144, null);
            node(helper, JOIN, Blocks.AIR, 0, 0, 0, null);
            node(helper, B, motor, -16, -16, 262144, null);
            expectedDrops = Map.of("create:shaft", 1);
        } else if (fixture.equals("arbitration_dominant_removal") && at > 70) {
            node(helper, A, motor, 16, 16, 262144, null);
            node(helper, JOIN, shaft, 16, 0, 262144, A);
            node(helper, B, Blocks.AIR, 0, 0, 0, null);
            expectedDrops = Map.of("create:creative_motor", 1);
        } else if (fixture.equals("arbitration_subordinate_reversal") && at > 70) {
            node(helper, A, Blocks.AIR, 0, 0, 0, null);
            node(helper, JOIN, shaft, 32, 0, 524288, B);
            node(helper, B, motor, 32, 32, 524288, null);
            expectedDrops = Map.of("create:creative_motor", 1);
        } else {
            boolean equal = fixture.equals("arbitration_equal_merge");
            float rpm = equal ? 16 : 32;
            float capacity = equal ? 524288 : 786432;
            node(helper, A, motor, rpm, 16, capacity, equal ? null : JOIN);
            node(helper, JOIN, shaft, rpm, 0, capacity, equal ? A : B);
            node(helper, B, motor, rpm, rpm, capacity, equal ? JOIN : null);
        }
        helper.assertTrue(drops(helper).equals(expectedDrops), "Wrong full live drop map for " + fixture);
    }

    private static void merge(GameTestHelper helper, String fixture, int targetB, int end) {
        floor(helper);
        helper.setBlock(A, AllBlocks.CREATIVE_MOTOR.getDefaultState().setValue(CreativeMotorBlock.FACING, Direction.UP));
        helper.setBlock(B, AllBlocks.CREATIVE_MOTOR.getDefaultState().setValue(CreativeMotorBlock.FACING, Direction.DOWN));
        for (int tick = 1; tick <= end; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                PortReferenceTests.sample(helper, fixture, Map.of("drops", drops(helper), "max_rpm",
                        AllConfigs.server().kinetics.maxRotationSpeed.get()), A, JOIN, B);
                if (at == 10) { request(helper, A, 16); request(helper, B, targetB); }
                if (at == 20) {
                    helper.assertTrue(PortReferenceTests.kinetic(helper, A).getGeneratedSpeed() == 16
                            && PortReferenceTests.kinetic(helper, B).getGeneratedSpeed() == targetB,
                            "Disconnected motor generated RPM must match fixture inputs");
                }
                if (at == 30) helper.setBlock(JOIN, PortReferenceTests.axial(AllBlocks.SHAFT.getDefaultState()));
                if (at == 70 && fixture.equals("arbitration_dominant_removal"))
                    helper.getLevel().destroyBlock(helper.absolutePos(B), true);
                if (at == 70 && fixture.equals("arbitration_subordinate_reversal")) request(helper, A, -16);
                if (at == 40 || at == 80 || at == end) outcome(helper, fixture, at);
                if (at == end) helper.succeed();
            });
        }
    }

    @GameTest(template = "empty", timeoutTicks = 160)
    public static void equalMerge(GameTestHelper helper) { merge(helper, "arbitration_equal_merge", 16, 120); }
    @GameTest(template = "empty", timeoutTicks = 160)
    public static void unequalMerge(GameTestHelper helper) { merge(helper, "arbitration_unequal_merge", 32, 120); }
    @GameTest(template = "empty", timeoutTicks = 160)
    public static void oppositeMerge(GameTestHelper helper) { merge(helper, "arbitration_opposite_merge", -16, 120); }
    @GameTest(template = "empty", timeoutTicks = 200)
    public static void dominantRemoval(GameTestHelper helper) { merge(helper, "arbitration_dominant_removal", 32, 160); }
    @GameTest(template = "empty", timeoutTicks = 200)
    public static void subordinateReversal(GameTestHelper helper) { merge(helper, "arbitration_subordinate_reversal", 32, 160); }

    @GameTest(template = "empty", timeoutTicks = 160)
    public static void largeSmallOverspeed(GameTestHelper helper) {
        floor(helper);
        helper.setBlock(A, AllBlocks.CREATIVE_MOTOR.getDefaultState().setValue(CreativeMotorBlock.FACING, Direction.UP));
        helper.setBlock(JOIN, PortReferenceTests.axial(AllBlocks.LARGE_COGWHEEL.getDefaultState()));
        BlockPos small = JOIN.offset(1, 0, 1);
        helper.setBlock(small, PortReferenceTests.axial(AllBlocks.COGWHEEL.getDefaultState()));
        helper.assertTrue(AllConfigs.server().kinetics.maxRotationSpeed.get() == 256, "Requires captured default 256 RPM ceiling");
        for (int tick = 1; tick <= 120; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                PortReferenceTests.sample(helper, "arbitration_large_small_overspeed", Map.of("drops", drops(helper),
                        "max_rpm", AllConfigs.server().kinetics.maxRotationSpeed.get()), A, JOIN, small);
                if (at == 30) {
                    helper.assertTrue(PortReferenceTests.kinetic(helper, small).getSpeed() == -32, "Initial gear ratio must settle");
                    request(helper, A, 256);
                }
                if (at == 40 || at == 120) {
                    node(helper, A, AllBlocks.CREATIVE_MOTOR.get(), 256, 256, 4194304, null);
                    node(helper, JOIN, Blocks.AIR, 0, 0, 0, null);
                    node(helper, small, AllBlocks.COGWHEEL.get(), 0, 0, 0, null);
                    helper.assertTrue(drops(helper).equals(Map.of("create:large_cogwheel", 1)), "Wrong overspeed drop outcome");
                }
                if (at == 120) helper.succeed();
            });
        }
    }
}
