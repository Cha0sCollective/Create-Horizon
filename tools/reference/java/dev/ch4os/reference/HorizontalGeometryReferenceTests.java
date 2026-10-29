package dev.ch4os.reference;

import com.simibubi.create.AllBlocks;
import com.simibubi.create.api.stress.BlockStressValues;
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
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.neoforged.neoforge.gametest.GameTestHolder;
import net.neoforged.neoforge.gametest.PrefixGameTestTemplate;

/** Four source-informed horizontal observations, not a native connection algorithm. */
@GameTestHolder("create_horizon_horizontal")
@PrefixGameTestTemplate(false)
public class HorizontalGeometryReferenceTests {
    private static BlockState axial(BlockState state, Direction.Axis axis) {
        return state.setValue(BlockStateProperties.AXIS, axis);
    }

    private static int[][] rotationBasis(GameTestHelper helper) {
        BlockPos origin = helper.absolutePos(BlockPos.ZERO);
        BlockPos[] basis = {new BlockPos(1, 0, 0), new BlockPos(0, 1, 0), new BlockPos(0, 0, 1)};
        int[][] observed = new int[3][];
        for (int i = 0; i < basis.length; i++) {
            BlockPos delta = helper.absolutePos(basis[i]).subtract(origin);
            observed[i] = new int[] {delta.getX(), delta.getY(), delta.getZ()};
        }
        return observed;
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

    private static void request(GameTestHelper helper, BlockPos pos, Direction facing, int generated) {
        var motor = (CreativeMotorBlockEntity) helper.getBlockEntity(pos);
        motor.generatedSpeed.setValue(facing.getAxisDirection() == Direction.AxisDirection.NEGATIVE
                ? -generated : generated);
    }

    private static void node(GameTestHelper helper, BlockPos pos, BlockState expectedState,
            float rpm, float generated, float capacity, BlockPos parent, BlockPos root) {
        helper.assertTrue(helper.getBlockState(pos).equals(expectedState), "Wrong block/properties at " + pos);
        helper.assertTrue(expectedState.canSurvive(helper.getLevel(), helper.absolutePos(pos)),
                "Invalid fixture placement at " + pos);
        var be = PortReferenceTests.kinetic(helper, pos);
        helper.assertTrue(be.getTheoreticalSpeed() == rpm && be.getSpeed() == rpm,
                "Wrong theoretical/effective RPM at " + pos);
        helper.assertTrue(be.getGeneratedSpeed() == generated, "Wrong generated RPM at " + pos);
        helper.assertTrue(parent == null ? be.source == null : helper.absolutePos(parent).equals(be.source),
                "Wrong source parent at " + pos);
        var network = be.saveWithoutMetadata(helper.getLevel().registryAccess()).getCompound("Network");
        helper.assertTrue(be.hasNetwork() == (capacity > 0) && network.getFloat("Capacity") == capacity
                && network.getFloat("Stress") == 0, "Wrong network capacity/stress at " + pos);
        if (capacity > 0) {
            var source = PortReferenceTests.kinetic(helper, root);
            helper.assertTrue(source.hasNetwork() && be.network.equals(source.network),
                    "Powered node is not in the motor network at " + pos);
        }
    }

    private static void horizontal(GameTestHelper helper, String fixture, boolean xAxis, boolean connected) {
        BlockPos motor = xAxis ? new BlockPos(2, 3, 3) : new BlockPos(3, 3, 4);
        BlockPos shaft = new BlockPos(3, 3, 3);
        BlockPos large = xAxis ? new BlockPos(4, 3, 3) : new BlockPos(3, 3, 2);
        BlockPos small = xAxis ? new BlockPos(4, 4, 4) : new BlockPos(4, 4, 2);
        Direction facing = xAxis ? Direction.EAST : Direction.NORTH;
        Direction.Axis axis = xAxis ? Direction.Axis.X : Direction.Axis.Z;
        Direction.Axis smallAxis = connected ? axis : xAxis ? Direction.Axis.Z : Direction.Axis.X;
        BlockState motorState = AllBlocks.CREATIVE_MOTOR.getDefaultState().setValue(CreativeMotorBlock.FACING, facing);
        BlockState shaftState = axial(AllBlocks.SHAFT.getDefaultState(), axis);
        BlockState largeState = axial(AllBlocks.LARGE_COGWHEEL.getDefaultState(), axis);
        BlockState smallState = axial(AllBlocks.COGWHEEL.getDefaultState(), smallAxis);
        for (int x = 0; x < 16; x++) for (int z = 0; z < 16; z++)
            helper.setBlock(new BlockPos(x, 0, z), Blocks.STONE);
        helper.assertTrue(java.util.Arrays.deepEquals(rotationBasis(helper),
                new int[][] {{1, 0, 0}, {0, 1, 0}, {0, 0, 1}}), "Requires actual GameTest rotation NONE");
        helper.setBlock(motor, motorState);
        helper.setBlock(shaft, shaftState);
        helper.setBlock(large, largeState);
        helper.setBlock(small, smallState);
        helper.assertTrue(largeState.canSurvive(helper.getLevel(), helper.absolutePos(large))
                && smallState.canSurvive(helper.getLevel(), helper.absolutePos(small)),
                "Both connected and mismatched cog layouts must be legal");
        double capacityPerRpm = BlockStressValues.getCapacity(AllBlocks.CREATIVE_MOTOR.get());
        helper.assertTrue(capacityPerRpm == 16384, "Requires captured motor capacity 16384 SU/RPM");
        helper.assertTrue(AllConfigs.server().kinetics.maxRotationSpeed.get() == 256,
                "Requires captured default 256 RPM ceiling");
        for (int tick = 1; tick <= 60; tick++) {
            final int at = tick;
            helper.runAtTickTime(tick, () -> {
                PortReferenceTests.sample(helper, fixture, Map.of("drops", drops(helper),
                        "max_rpm", AllConfigs.server().kinetics.maxRotationSpeed.get(),
                        "motor_capacity_per_rpm", capacityPerRpm, "rotation_basis", rotationBasis(helper)),
                        motor, shaft, large, small);
                if (at == 10) request(helper, motor, facing, 16);
                if (at == 30) request(helper, motor, facing, -16);
                if (at == 20 || at == 50 || at == 60) {
                    float generated = at == 20 ? 16 : -16;
                    int setting = facing == Direction.NORTH ? -(int) generated : (int) generated;
                    helper.assertTrue(((CreativeMotorBlockEntity) helper.getBlockEntity(motor)).generatedSpeed.getValue() == setting,
                            "Wrong oriented motor setting");
                    node(helper, motor, motorState, generated, generated, 262144, null, motor);
                    node(helper, shaft, shaftState, generated, 0, 262144, motor, motor);
                    node(helper, large, largeState, generated, 0, 262144, shaft, motor);
                    node(helper, small, smallState, connected ? -2 * generated : 0, 0,
                            connected ? 262144 : 0, connected ? large : null, motor);
                    helper.assertTrue(drops(helper).isEmpty(), "Unexpected item drop in horizontal geometry fixture");
                }
                if (at == 60) helper.succeed();
            });
        }
    }

    @GameTest(template = "empty", timeoutTicks = 100, rotationSteps = 0)
    public static void xConnected(GameTestHelper helper) { horizontal(helper, "horizontal_x_connected", true, true); }
    @GameTest(template = "empty", timeoutTicks = 100, rotationSteps = 0)
    public static void xMismatch(GameTestHelper helper) { horizontal(helper, "horizontal_x_mismatch", true, false); }
    @GameTest(template = "empty", timeoutTicks = 100, rotationSteps = 0)
    public static void zConnected(GameTestHelper helper) { horizontal(helper, "horizontal_z_connected", false, true); }
    @GameTest(template = "empty", timeoutTicks = 100, rotationSteps = 0)
    public static void zMismatch(GameTestHelper helper) { horizontal(helper, "horizontal_z_mismatch", false, false); }
}
