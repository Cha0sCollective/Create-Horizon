#!/usr/bin/env python3
"""Generate the small playable content set with original, provisional models.

Models reference vanilla textures by ID and contain no upstream texture bytes.
The full horizon-port-pack carry-over remains in .local for subsequent milestones.
"""
import json
from common import ROOT, NATIVE
from fluid_pump_content import generate as fluid_pump_content

CONTENT = ROOT / "content"
BLOCKS = ["creative_motor", "shaft", "cogwheel", "large_cogwheel", "mechanical_press", "depot"]
SHEETS = ["iron", "golden", "copper"]


def write(path, value):
    target = CONTENT / path
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def model(name, boxes, texture="minecraft:block/iron_block"):
    faces = {face: {"texture": "#all"} for face in ["down", "up", "north", "south", "west", "east"]}
    write("assets/create/models/block/" + name + ".json", {
        "textures": {"all": texture, "particle": texture},
        "elements": [{"from": a, "to": b, "faces": faces} for a, b in boxes],
    })

def mechanical_piston_content():
    # Finite normal piston profile. These are our fallback geometry and vanilla
    # textures; the player's 6.0.10 JAR supplies original Create visuals.
    piston = {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 1.5, "explosion_resistance": 6,
                       "requires_tool": True, "render_shape": "MODEL",
                       "sounds": "minecraft:andesite"},
        "state_properties": {
            "facing": {"type": "enum", "values": ["down", "up", "north", "south", "west", "east"]},
            "axis_along_first": {"type": "boolean"},
            "state": {"type": "enum", "values": ["retracted", "moving", "extended"]},
        },
        "default_state": {"facing": "east", "axis_along_first": "false", "state": "retracted"},
        "collision_shapes": {"default": [[0, 0, 0, 1, 1, 1]]},
    }
    pole = {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 1.5, "render_shape": "MODEL", "sounds": "minecraft:scaffolding"},
        "state_properties": {"facing": {"type": "enum", "values": ["down", "up", "north", "south", "west", "east"]},
                             "waterlogged": {"type": "boolean"}},
        "default_state": {"facing": "east", "waterlogged": "false"},
        "shapes": {"default": [[0, .375, .375, 1, .625, .625]]},
        "collision_shapes": {"default": [[0, .375, .375, 1, .625, .625]]},
    }
    head = {
        "template": "minecraft:glass", "item": False,
        "properties": {"hardness": 1.5, "render_shape": "MODEL", "sounds": "minecraft:scaffolding"},
        "state_properties": {"facing": {"type": "enum", "values": ["down", "up", "north", "south", "west", "east"]},
                             "type": {"type": "enum", "values": ["normal", "sticky"]},
                             "waterlogged": {"type": "boolean"}},
        "default_state": {"facing": "east", "type": "normal", "waterlogged": "false"},
        "shapes": {"default": [[0, .375, .375, .75, .625, .625], [.75, 0, 0, 1, 1, 1]]},
        "collision_shapes": {"default": [[0, .375, .375, .75, .625, .625], [.75, 0, 0, 1, 1, 1]]},
    }
    for name, declaration in [("mechanical_piston", piston),
                              ("piston_extension_pole", pole),
                              ("mechanical_piston_head", head)]:
        write("data/create/horizon/block/" + name + ".json", declaration)
    model("mechanical_piston/normal/block", [([0, 0, 0], [16, 16, 16])], "minecraft:block/andesite")
    model("piston_extension_pole", [([0, 6, 6], [16, 10, 10])], "minecraft:block/iron_block")
    model("mechanical_piston/normal/head", [([0, 6, 6], [12, 10, 10]),
                                            ([12, 0, 0], [16, 16, 16])], "minecraft:block/andesite")
    for name, part in [("mechanical_piston", "mechanical_piston/normal/block"),
                       ("piston_extension_pole", "piston_extension_pole"),
                       ("mechanical_piston_head", "mechanical_piston/normal/head")]:
        write("assets/create/blockstates/" + name + ".json",
              {"variants": {"": {"model": "create:block/" + part}}})
        if name != "mechanical_piston_head":
            write("assets/create/models/item/" + name + ".json", {"parent": "create:block/" + part})
            write("assets/create/items/" + name + ".json",
                  {"model": {"type": "minecraft:model", "model": "create:item/" + name}})
        write("data/create/loot_table/blocks/" + name + ".json",
              {"type": "minecraft:block", "pools": [{"rolls": 1, "entries": [
                  {"type": "minecraft:item", "name": "create:" +
                   ("piston_extension_pole" if name == "mechanical_piston_head" else name)}]}]})


def portable_interface_content():
    # MIT AllShapes and WrenchableDirectionalBlock: FACING only, 14/16 high
    # before rotation. Provisional art below is authored here with vanilla IDs.
    directions = ["down", "up", "north", "south", "west", "east"]
    shapes = {"up": [[0, 0, 0, 1, .875, 1]], "down": [[0, .125, 0, 1, 1, 1]],
              "north": [[0, 0, .125, 1, 1, 1]], "south": [[0, 0, 0, 1, 1, .875]],
              "west": [[.125, 0, 0, 1, 1, 1]], "east": [[0, 0, 0, .875, 1, 1]]}
    write("data/create/horizon/block/portable_storage_interface.json", {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 1.5, "explosion_resistance": 6,
                       "requires_tool": True, "render_shape": "MODEL", "sounds": "minecraft:andesite"},
        "state_properties": {"facing": {"type": "enum", "values": directions}},
        "default_state": {"facing": "east"},
        "shapes": {"property": "facing", "values": shapes},
        "collision_shapes": {"property": "facing", "values": shapes},
    })
    base = "portable_storage_interface/"
    model(base + "block", [([0, 0, 0], [16, 8, 16])], "minecraft:block/andesite")
    model(base + "block_middle", [([4, 0, 4], [12, 6, 12])], "minecraft:block/iron_block")
    model(base + "block_middle_powered", [([4, 0, 4], [12, 6, 12])], "minecraft:block/gold_block")
    model(base + "block_top", [([1, 12, 1], [15, 14, 15])], "minecraft:block/iron_block")
    model(base + "item", [([0, 0, 0], [16, 8, 16]), ([4, 6, 4], [12, 12, 12]),
                           ([1, 12, 1], [15, 14, 15])], "minecraft:block/andesite")
    rotations = {"up": {}, "down": {"x": 180}, "north": {"x": 90},
                 "south": {"x": 90, "y": 180}, "west": {"x": 90, "y": 270},
                 "east": {"x": 90, "y": 90}}
    write("assets/create/blockstates/portable_storage_interface.json", {"variants": {
        "facing=" + face: {"model": "create:block/" + base + "block", **rotation}
        for face, rotation in rotations.items()}})
    write("assets/create/models/item/portable_storage_interface.json", {"parent": "create:block/" + base + "item"})
    write("assets/create/items/portable_storage_interface.json", {"model": {
        "type": "minecraft:model", "model": "create:item/portable_storage_interface"}})
    write("data/create/loot_table/blocks/portable_storage_interface.json", {
        "type": "minecraft:block", "pools": [{"rolls": 1, "entries": [
            {"type": "minecraft:item", "name": "create:portable_storage_interface"}]}]})
    for locale in ["en_us", "ru_ru"]:
        path = CONTENT / ("assets/create/lang/" + locale + ".json")
        lang = json.loads(path.read_text())
        lang["block.create.portable_storage_interface"] = "Portable Storage Interface"
        path.write_text(json.dumps(lang, ensure_ascii=False, indent=2) + "\n")


def straight_track_content():
    # Selected stationary Create TrackBlock state. Graphs, curves, turns and
    # water placement are not implemented. Original art is JAR-only.
    write("data/create/horizon/block/track.json", {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 0.8, "explosion_resistance": 0.8,
                       "requires_tool": True, "render_shape": "MODEL",
                       "sounds": "minecraft:iron_block"},
        "state_properties": {
            "shape": {"type": "enum", "values": ["xo", "zo"]},
            "turn": {"type": "boolean"},
            "waterlogged": {"type": "boolean"},
        },
        "default_state": {"shape": "zo", "turn": "false", "waterlogged": "false"},
        # Source collision is a flat 16x2x16 plate. The provisional outline
        # remains inside one cell; source selection extends into neighbors.
        "shapes": {"default": [[0, 0, 0, 1, 0.125, 1]]},
        "collision_shapes": {"default": [[0, 0, 0, 1, 0.125, 1]]},
    })
    model("track/x_ortho", [([0, 0, 2], [16, 2, 4]),
                            ([0, 0, 12], [16, 2, 14])], "minecraft:block/iron_block")
    model("track/z_ortho", [([2, 0, 0], [4, 2, 16]),
                            ([12, 0, 0], [14, 2, 16])], "minecraft:block/iron_block")
    write("assets/create/blockstates/track.json", {"variants": {
        f"shape={shape},turn={turn},waterlogged={water}": {
            "model": f"create:block/track/{'x' if shape == 'xo' else 'z'}_ortho"
        }
        for shape in ("xo", "zo") for turn in ("false", "true")
        for water in ("false", "true")
    }})
    write("assets/create/models/item/track.json", {
        "parent": "minecraft:item/generated",
        "textures": {"layer0": "minecraft:item/iron_nugget"},
    })
    write("assets/create/items/track.json", {"model": {
        "type": "minecraft:model", "model": "create:item/track"}})
    write("data/create/loot_table/blocks/track.json", {
        "type": "minecraft:block", "pools": [{"rolls": 1, "entries": [
            {"type": "minecraft:item", "name": "create:track"}]}]})
    write("data/minecraft/tags/block/mineable/pickaxe.json", {
        "replace": False, "values": ["create:track"]})


def dry_mixing_content():
    # MIT declarations adapted from upstream236c87a8. Contextual shapes/contact
    # are intentionally absent on the pinned host; this profile is hopper-fed.
    declarations = {'basin': {'template': 'minecraft:glass', 'item': True, 'properties': {'hardness': 1.5, 'explosion_resistance': 6, 'requires_tool': True, 'map_color': 21, 'sounds': 'minecraft:netherite_block', 'render_shape': 'MODEL'}, 'state_properties': {'facing': {'type': 'enum', 'values': ['down', 'north', 'south', 'west', 'east']}}, 'default_state': {'facing': 'down'}, 'shapes': {'default': [[0, 0.125, 0, 0.125, 1, 1], [0.875, 0.125, 0, 1, 1, 1], [0.125, 0.125, 0, 0.875, 1, 0.125], [0.125, 0.125, 0.875, 0.875, 1, 1], [0.125, 0, 0.125, 0.875, 0.125, 0.875]]}, 'collision_shapes': {'default': [[0, 0.125, 0, 0.125, 1, 1], [0.875, 0.125, 0, 1, 1, 1], [0.125, 0.125, 0, 0.875, 1, 0.125], [0.125, 0.125, 0.875, 0.875, 1, 1], [0.125, 0, 0.125, 0.875, 0.125, 0.875]]}}, 'mechanical_mixer': {'template': 'minecraft:glass', 'item': True, 'properties': {'hardness': 1.5, 'explosion_resistance': 6, 'requires_tool': True, 'map_color': 11, 'sounds': 'minecraft:stone', 'render_shape': 'MODEL'}, 'shapes': {'default': [[0, 0, 0, 0.25, 1, 1], [0.75, 0, 0, 1, 1, 1], [0.25, 0, 0, 0.75, 1, 0.25], [0.25, 0, 0.75, 0.75, 1, 1]]}, 'collision_shapes': {'default': [[0, 0, 0, 0.25, 1, 1], [0.75, 0, 0, 1, 1, 1], [0.25, 0, 0, 0.75, 1, 0.25], [0.25, 0, 0.75, 0.75, 1, 1]]}}}
    for name, declaration in declarations.items():
        write("data/create/horizon/block/" + name + ".json", declaration)
        write("assets/create/blockstates/" + name + ".json", {"variants": {"": {"model": "create:block/" + name + "/block"}}})
        write("assets/create/models/item/" + name + ".json", {"parent": "create:block/" + name + "/block"})
        write("assets/create/items/" + name + ".json", {"model": {"type": "minecraft:model", "model": "create:item/" + name}})
        write("data/create/loot_table/blocks/" + name + ".json", {"type": "minecraft:block", "pools": [{"rolls": 1, "entries": [{"type": "minecraft:item", "name": "create:" + name}]}]})
    model("basin/block", [([0,2,0],[2,16,16]),([14,2,0],[16,16,16]),([2,2,0],[14,16,2]),([2,2,14],[14,16,16]),([2,0,2],[14,2,14])], "minecraft:block/andesite")
    model("mechanical_mixer/block", [([0,0,0],[4,16,16]),([12,0,0],[16,16,16]),([4,0,0],[12,16,4]),([4,0,12],[12,16,16])], "minecraft:block/andesite")
    model("cogwheel_shaftless", [([0,6,0],[16,10,16])])
    model("mechanical_mixer/pole", [([6,0,6],[10,16,10])])
    model("mechanical_mixer/head", [([2,0,2],[14,3,14]),([6,3,6],[10,8,10])])
    write("data/create/recipe/mixing/andesite_alloy.json", {"type":"create:mixing", "ingredients":[{"item":"minecraft:andesite"},{"tag":"c:nuggets/iron"}], "results":[{"id":"create:andesite_alloy"}]})
    tag_path=CONTENT / "data/c/tags/item/nuggets/iron.json"
    tag=json.loads(tag_path.read_text()) if tag_path.exists() else {"values":[]}
    if "minecraft:iron_nugget" not in tag["values"]: tag["values"].append("minecraft:iron_nugget")
    write("data/c/tags/item/nuggets/iron.json",tag)
    for locale in ["en_us", "ru_ru"]:
        path=CONTENT / ("assets/create/lang/" + locale + ".json")
        lang=json.loads(path.read_text());lang.update({"block.create.basin":"Basin", "block.create.mechanical_mixer":"Mechanical Mixer"})
        path.write_text(json.dumps(lang, indent=2, ensure_ascii=False) + "\n")


def processors():
    # State and collision definitions translated from MIT MillstoneBlock,
    # DirectionalKineticBlock and AllShapes. Geometry below is our provisional
    # design using vanilla textures, not a copy of the original ARR models.
    shapes = {"default": [[0, 0, 0, 1, 6 / 16, 1], [2 / 16, 6 / 16, 2 / 16, 14 / 16, 1, 14 / 16]]}
    write("data/create/horizon/block/millstone.json", {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 1.5, "explosion_resistance": 6, "requires_tool": True, "map_color": 6,
                       "render_shape": "MODEL", "sounds": "minecraft:andesite"},
        "shapes": shapes, "collision_shapes": shapes,
    })
    full = {"default": [[0, 0, 0, 1, 1, 1]]}
    write("data/create/horizon/block/encased_fan.json", {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 1.5, "explosion_resistance": 6, "requires_tool": True, "map_color": 34,
                       "render_shape": "MODEL", "sounds": "minecraft:andesite"},
        "state_properties": {"facing": {"type": "enum", "values": ["down", "up", "north", "south", "west", "east"]}},
        # 1.21.1 DirectionProperty's implicit first value is DOWN. The source
        # fan does not override it; placement supplies its actual direction.
        "default_state": {"facing": "down"}, "shapes": full, "collision_shapes": full,
    })
    drill_shapes = {"property": "facing", "values": {
        "up": [[0, 0, 0, 1, .75, 1]], "down": [[0, .25, 0, 1, 1, 1]],
        "north": [[0, 0, .25, 1, 1, 1]], "south": [[0, 0, 0, 1, 1, .75]],
        "west": [[.25, 0, 0, 1, 1, 1]], "east": [[0, 0, 0, .75, 1, 1]],
    }}
    write("data/create/horizon/block/mechanical_drill.json", {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 1.5, "explosion_resistance": 6, "requires_tool": True,
                       "map_color": 34, "render_shape": "MODEL", "sounds": "minecraft:andesite"},
        "state_properties": {"facing": {"type": "enum", "values": ["down", "up", "north", "south", "west", "east"]},
                             "waterlogged": {"type": "boolean"}},
        "default_state": {"facing": "down", "waterlogged": "false"},
        "shapes": drill_shapes, "collision_shapes": drill_shapes,
    })
    model("millstone/block", [([0, 0, 0], [16, 5, 16]),
                               ([2, 5, 2], [5, 16, 14]), ([11, 5, 2], [14, 16, 14]),
                               ([5, 5, 2], [11, 16, 5]), ([5, 5, 11], [11, 16, 14])], "minecraft:block/andesite")
    model("millstone/inner", [([5, 5, 5], [11, 12, 11]), ([6, 12, 7], [10, 15, 9])], "minecraft:block/smooth_stone")
    # The source static fan model faces up before blockstate rotations.
    model("encased_fan/block", [([0, 0, 0], [16, 3, 16]),
                                 ([0, 3, 0], [3, 16, 16]), ([13, 3, 0], [16, 16, 16]),
                                 ([3, 3, 0], [13, 16, 3]), ([3, 3, 13], [13, 16, 16])], "minecraft:block/stripped_spruce_log")
    # The native renderer faces the moving partial along the rear shaft.
    model("encased_fan/propeller", [([3, 6, 7], [13, 10, 9]), ([6, 3, 7], [10, 13, 9]), ([6, 6, 5], [10, 10, 11])])
    model("mechanical_drill/block", [([0, 0, 0], [16, 10, 16]), ([2, 10, 2], [14, 12, 14])], "minecraft:block/andesite")
    model("mechanical_drill/head", [([4, 12, 4], [12, 13, 12]), ([5, 13, 5], [11, 14, 11]), ([7, 14, 7], [9, 16, 9])])
    for block in ["millstone", "encased_fan", "mechanical_drill"]:
        variants = {"": {"model": "create:block/" + block + "/block"}}
        if block != "millstone":
            variants = {"facing=" + face: {"model": "create:block/" + block + "/block", **rotation}
                        for face, rotation in {"down": {"x": 180}, "up": {}, "north": {"x": 90},
                                               "south": {"x": 90, "y": 180}, "west": {"x": 90, "y": 270},
                                               "east": {"x": 90, "y": 90}}.items()}
            if block == "mechanical_drill":
                variants = {key + ",waterlogged=" + wet: value for key, value in variants.items() for wet in ["false", "true"]}
        write("assets/create/blockstates/" + block + ".json", {"variants": variants})
        write("assets/create/models/item/" + block + ".json", {"parent": "create:block/" + block + "/block"})
        write("assets/create/items/" + block + ".json", {"model": {"type": "minecraft:model", "model": "create:item/" + block}})
        write("data/create/loot_table/blocks/" + block + ".json", {"type": "minecraft:block", "pools": [{"rolls": 1, "entries": [{"type": "minecraft:item", "name": "create:" + block}]}]})


def foundation():
    from catalog import BLOCKS as all_blocks, ITEMS, TRANSMISSIONS, STATIC_BLOCKS, INSTRUMENTS, GENERATORS, INTERNAL_BLOCKS, rust_catalog
    for block in TRANSMISSIONS + STATIC_BLOCKS + INSTRUMENTS + GENERATORS:
        declaration = {"template": "minecraft:glass", "item": True,
                       "properties": {"hardness": 1.5, "sounds": "minecraft:andesite"}}
        if block in TRANSMISSIONS and block != "hand_crank":
            declaration["state_properties"] = {"axis": {"type": "enum", "values": ["x", "y", "z"]}}
            declaration["default_state"] = {"axis": "y"}
            if block in ["clutch", "gearshift"]:
                declaration["state_properties"]["powered"] = {"type": "boolean"}
                declaration["default_state"]["powered"] = "false"
        if block in INSTRUMENTS:
            declaration['state_properties']={'facing':{'type':'enum','values':['down','up','north','south','west','east']},'axis_along_first':{'type':'boolean'}}
            declaration['default_state']={'facing':'north','axis_along_first':'false'}
        if block in GENERATORS:
            declaration['properties']['render_shape']='ENTITYBLOCK_ANIMATED' if block=='large_water_wheel' else 'MODEL'
            declaration['properties']['sounds']='minecraft:stripped_spruce_wood'
            key='axis' if block=='large_water_wheel' else 'facing'
            values=['x','y','z'] if key=='axis' else ['down','up','north','south','west','east']
            declaration['state_properties']={key:{'type':'enum','values':values}}
            declaration['default_state']={key:'x' if key=='axis' else 'east'}
            if key=='axis':
                declaration['state_properties']['extension']={'type':'boolean'}
                declaration['default_state']['extension']='false'
        if block == "hand_crank":
            declaration["properties"]["sounds"] = "minecraft:stripped_spruce_wood"
            declaration["properties"]["render_shape"] = "ENTITYBLOCK_ANIMATED"
            declaration["state_properties"] = {"facing": {"type": "enum", "values": ["up", "down", "north", "south", "west", "east"]}, "waterlogged": {"type": "boolean"}}
            declaration["default_state"] = {"facing": "up", "waterlogged": "false"}
        write("data/create/horizon/block/" + block + ".json", declaration)
        texture = "minecraft:block/stripped_spruce_log" if "casing" in block else "minecraft:block/iron_block"
        model(block + "/block", [([0, 0, 0], [16, 16, 16])], texture)
        write("assets/create/blockstates/" + block + ".json", {"variants": {"": {"model": "create:block/" + block + "/block"}}})
        write("assets/create/models/item/" + block + ".json", {"parent": "create:block/" + block + "/block"})
        write("assets/create/items/" + block + ".json", {"model": {"type": "minecraft:model", "model": "create:item/" + block}})
        drop = "shaft" if "encased_shaft" in block else block
        write("data/create/loot_table/blocks/" + block + ".json", {"type": "minecraft:block", "pools": [{"rolls": 1, "entries": [{"type": "minecraft:item", "name": "create:" + drop}]}]})
    write('data/create/horizon/block/water_wheel_structure.json', {'template':'minecraft:glass','item':False,'properties':{'hardness':1.5,'render_shape':'INVISIBLE','sounds':'minecraft:stripped_spruce_wood'},'state_properties':{'facing':{'type':'enum','values':['down','up','north','south','west','east']}},'default_state':{'facing':'north'}})
    write('assets/create/blockstates/water_wheel_structure.json',{'variants':{'':{'model':'minecraft:block/air'}}})
    write('data/create/loot_table/blocks/water_wheel_structure.json',{'type':'minecraft:block','pools':[]})
    for item in ITEMS:
        if item in [name + '_sheet' for name in SHEETS]: continue
        item_decl={"template":"minecraft:paper" if item=='schematic_and_quill' else "minecraft:iron_ingot",
                   "max_stack_size":1 if item in ['wrench','goggles','schematic_and_quill'] else 64}
        if item=='goggles':item_decl['components']={'minecraft:equippable':{'slot':'head','equip_sound':'minecraft:item.armor.equip_generic','dispensable':True,'swappable':True,'damage_on_hurt':False}}
        write("data/create/horizon/item/" + item + ".json", item_decl)
        sprite = {"wheat_flour": "sugar", "dough": "bread",
                  "schematic_and_quill": "feather"}.get(item, "iron_ingot")
        write("assets/create/models/item/" + item + ".json", {"parent": "minecraft:item/generated", "textures": {"layer0": "minecraft:item/" + sprite}})
        write("assets/create/items/" + item + ".json", {"model": {"type": "minecraft:model", "model": "create:item/" + item}})
        if item=='goggles':
            write('assets/create/items/goggles.json',{'model':{'type':'minecraft:select','property':'minecraft:display_context','cases':[{'when':'head','model':{'type':'minecraft:model','model':'create:block/goggles'}}],'fallback':{'type':'minecraft:model','model':'create:item/goggles'}}})
    vertical={'condition':'minecraft:block_state_property','block':'create:gearbox','properties':{'axis':'y'}}
    write('data/create/loot_table/blocks/gearbox.json',{'type':'minecraft:block','pools':[{'rolls':1,'entries':[{'type':'minecraft:alternatives','children':[{'type':'minecraft:item','name':'create:gearbox','conditions':[vertical]},{'type':'minecraft:item','name':'create:vertical_gearbox'}]}]}]})
    model("hand_crank/handle", [([6, 5, 6], [10, 10, 16])], "minecraft:block/oak_planks")
    model('goggles',[([4,6,3],[7,9,4]),([9,6,3],[12,9,4])],'minecraft:block/gold_block')
    processors()
    # A native world-space hint: our own geometry and vanilla sprite reference.
    model('port_hint',[([7.6,7.6,5],[8.4,8.4,14]),([6,7.5,11],[10,8.5,12]),([7,7.5,12],[9,8.5,13])],'minecraft:block/white_concrete')
    hint=CONTENT/'assets/create/models/block/port_hint.json'
    value=json.loads(hint.read_text())
    for element in value['elements']:
        for face in element['faces'].values():face['tintindex']=0
    hint.write_text(json.dumps(value,indent=2)+'\n')
    # Only MIT recipe/tag data is distributed; original assets stay in the JAR.
    reference = json.loads((ROOT / "tools/data/recipes.json").read_text())
    def current(value):
        if isinstance(value, dict):
            if set(value) == {"item"}: return value["item"]
            if set(value) == {"tag"}: return "#" + value["tag"]
            return {k: current(v) for k,v in value.items()}
        if isinstance(value, list): return [current(v) for v in value]
        return value
    for folder, retained in [(CONTENT / "data/create/recipe", reference["recipes"]),
                             (CONTENT / "data/create/tags", reference["tags"]),
                             (CONTENT / "data/c/tags/item", reference["tags"])]:
        for path in folder.rglob("*.json"):
            if path.relative_to(CONTENT / "data").as_posix() not in retained: path.unlink()
    for path, value in reference["recipes"].items(): write("data/" + path, current(value))
    for path, value in reference["tags"].items(): write("data/" + path, value)
    write("data/create/horizon/creative_tab/kinetics.json", {"title": "itemGroup.create.base", "icon": "create:cogwheel", "items": ["create:" + name for name in all_blocks + ITEMS]})
    # These fallback labels are ours; the JAR replaces them with all upstream locales.
    extra_blocks = {"gearbox":"Gearbox","clutch":"Clutch","gearshift":"Gearshift","hand_crank":"Hand Crank","andesite_encased_shaft":"Andesite Encased Shaft","brass_encased_shaft":"Brass Encased Shaft","andesite_casing":"Andesite Casing","brass_casing":"Brass Casing","andesite_alloy_block":"Block of Andesite Alloy","brass_block":"Block of Brass","zinc_block":"Block of Zinc","speedometer":"Speedometer","stressometer":"Stressometer","water_wheel":"Water Wheel","large_water_wheel":"Large Water Wheel","millstone":"Millstone","encased_fan":"Encased Fan","mechanical_drill":"Mechanical Drill","mechanical_piston":"Mechanical Piston","piston_extension_pole":"Piston Extension Pole","mechanical_piston_head":"Mechanical Piston Head","track":"Train Track"}
    extra_blocks.update({"mechanical_pump": "Mechanical Pump", "fluid_pipe": "Fluid Pipe"})
    extra_items = {"andesite_alloy":"Andesite Alloy","brass_ingot":"Brass Ingot","brass_nugget":"Brass Nugget","zinc_ingot":"Zinc Ingot","zinc_nugget":"Zinc Nugget","wrench":"Wrench","brass_sheet":"Brass Sheet","vertical_gearbox":"Vertical Gearbox","goggles":"Engineer’s Goggles","propeller":"Propeller","wheat_flour":"Wheat Flour","dough":"Dough","schematic_and_quill":"Schematic and Quill"}
    for locale in ["en_us","ru_ru"]:
        path=CONTENT / ("assets/create/lang/"+locale+".json")
        lang=json.loads(path.read_text())
        lang.update({"block.create."+k:v for k,v in extra_blocks.items()})
        lang.update({"item.create."+k:v for k,v in extra_items.items()})
        if locale == "ru_ru":
            lang.update({"block.create.millstone": "Мельничный жернов", "block.create.encased_fan": "Вентилятор в корпусе", "block.create.mechanical_drill": "Механический бур",
                         "block.create.mechanical_pump": "Механический насос", "block.create.fluid_pipe": "Жидкостная труба",
                         "item.create.propeller": "Пропеллер", "item.create.wheat_flour": "Пшеничная мука", "item.create.dough": "Тесто"})
        lang["create.kinetics.creative_motor.rotation_speed"]="Generated Speed in RPM"
        path.write_text(json.dumps(lang,ensure_ascii=False,indent=2)+"\n")
    (NATIVE / "assets/src/catalog.rs").write_text(rust_catalog())
    # Provisional aliases derived from MIT AllSoundEvents registrations.
    write("assets/create/sounds.json", {key: {"sounds": [{"type": "event", "name": event}]} for key,event in {
        "depot_slide": "minecraft:block.sand.break", "depot_plop": "minecraft:entity.item_frame.add_item",
        "mechanical_press_activation": "minecraft:block.anvil.land", "cranking": "minecraft:block.wood.place",
        "wrench_rotate": "minecraft:block.wooden_trapdoor.close", "wrench_remove": "minecraft:entity.item.pickup",
    }.items()})



def coal_heating_content():
    # Selected MIT upstream236c87a8 declarations. Original art remains JAR-only.
    write("data/create/horizon/block/blaze_burner.json", {'template': 'minecraft:glass', 'shapes': {'default': [[0.0625, 0, 0.0625, 0.9375, 0.875, 0.9375]]}, 'collision_shapes': {'default': [[0, 0, 0, 1, 0.25, 1]]}, 'collision_context_shapes': {'player': {'default': [[0.0625, 0, 0.0625, 0.9375, 0.875, 0.9375]]}, 'item': {'default': [[0.0625, 0, 0.0625, 0.9375, 0.875, 0.9375]]}, 'living': {'default': [[0.0625, 0, 0.0625, 0.9375, 0.875, 0.9375]]}, 'other': {'default': [[0.0625, 0, 0.0625, 0.9375, 0.875, 0.9375]]}}, 'item': True, 'properties': {'hardness': 3, 'explosion_resistance': 6, 'requires_tool': True, 'map_color': 21, 'sounds': 'minecraft:gold_block', 'render_shape': 'MODEL'}, 'state_properties': {'blaze': {'type': 'enum', 'values': ['none', 'smouldering', 'fading', 'kindled', 'seething']}, 'facing': {'type': 'enum', 'values': ['north', 'south', 'west', 'east']}}, 'default_state': {'blaze': 'none', 'facing': 'north'}, 'state_light': {'property': 'blaze', 'values': {'none': 0, 'smouldering': 8, 'fading': 15, 'kindled': 15, 'seething': 15}}})
    write("data/create/loot_table/blocks/blaze_burner.json", {'type': 'minecraft:block', 'pools': [{'rolls': 1, 'entries': [{'type': 'minecraft:item', 'name': 'create:empty_blaze_burner', 'conditions': [{'type': 'minecraft:survives_explosion'}, {'type': 'minecraft:block_state_property', 'block': 'create:blaze_burner', 'properties': {'blaze': 'none'}}]}, {'type': 'minecraft:item', 'name': 'create:blaze_burner', 'conditions': [{'type': 'minecraft:survives_explosion'}, {'type': 'minecraft:block_state_property', 'block': 'create:blaze_burner', 'properties': {'blaze': 'smouldering'}}]}, {'type': 'minecraft:item', 'name': 'create:blaze_burner', 'conditions': [{'type': 'minecraft:survives_explosion'}, {'type': 'minecraft:block_state_property', 'block': 'create:blaze_burner', 'properties': {'blaze': 'fading'}}]}, {'type': 'minecraft:item', 'name': 'create:blaze_burner', 'conditions': [{'type': 'minecraft:survives_explosion'}, {'type': 'minecraft:block_state_property', 'block': 'create:blaze_burner', 'properties': {'blaze': 'kindled'}}]}, {'type': 'minecraft:item', 'name': 'create:blaze_burner', 'conditions': [{'type': 'minecraft:survives_explosion'}, {'type': 'minecraft:block_state_property', 'block': 'create:blaze_burner', 'properties': {'blaze': 'seething'}}]}]}]})
    write("data/create/horizon/item/empty_blaze_burner.json", {"template":"minecraft:iron_ingot","max_stack_size":64,"place_block":"create:blaze_burner"})
    write("assets/create/blockstates/blaze_burner.json", {"variants":{"":{"model":"create:block/blaze_burner/block"}}})
    for name in ["blaze_burner", "empty_blaze_burner"]:
        write("assets/create/models/item/"+name+".json", {"parent":"create:block/blaze_burner/block"})
        write("assets/create/items/"+name+".json", {"model":{"type":"minecraft:model","model":"create:item/"+name}})
    # Authored fallback geometry uses vanilla textures, never copied source art.
    model("blaze_burner/block", [([0,0,0],[16,4,16]),([0,4,0],[2,14,2]),([14,4,0],[16,14,2]),([0,4,14],[2,14,16]),([14,4,14],[16,14,16])])
    for name in ["inert", "idle", "active"]:
        model("blaze_burner/blaze/"+name, [([4,5,4],[12,13,12])], "minecraft:block/gold_block")
    for name in ["rods_small", "rods_large"]:
        model("blaze_burner/"+name, [([2,4,2],[4,12,4]),([12,4,12],[14,12,14])], "minecraft:block/gold_block")
    model("blaze_burner/flame", [([4,0,4],[12,4,12])], "minecraft:block/magma")
    write("data/create/recipe/mixing/brass_ingot.json", {"type":"create:mixing","heat_requirement":"heated","ingredients":["#c:ingots/copper","#c:ingots/zinc"],"results":[{"count":2,"id":"create:brass_ingot"}]})
    for language in ["en_us", "ru_ru"]:
        path=CONTENT / ("assets/create/lang/"+language+".json")
        lang=json.loads(path.read_text());lang.update({"block.create.blaze_burner":"Blaze Burner","item.create.empty_blaze_burner":"Empty Blaze Burner"})
        write("assets/create/lang/"+language+".json",lang)
    path=CONTENT / "data/create/horizon/creative_tab/kinetics.json"
    tab=json.loads(path.read_text())
    for name in ["create:blaze_burner", "create:empty_blaze_burner"]:
        if name not in tab["items"]: tab["items"].append(name)
    write("data/create/horizon/creative_tab/kinetics.json",tab)


def belt_content():
    # Selected MIT7db source-authored declarations/fallbacks; local original art only.
    values = json.loads((ROOT / "tools/data/belt-content.json").read_text())
    for path, value in values.items(): write(path, value)


def fluid_tank_content():
    # One isolated controller. Source-style bottom/top/shape keys permit the
    # locally imported 6.0.10 blockstate to select its single-window model.
    name = "fluid_tank"
    write("data/create/horizon/block/" + name + ".json", {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 3, "explosion_resistance": 6,
                       "requires_tool": True, "render_shape": "MODEL",
                       "sounds": "minecraft:copper_block"},
        "state_properties": {
            "bottom": {"type": "boolean"},
            "top": {"type": "boolean"},
            "shape": {"type": "enum", "values": ["window", "plain"]},
        },
        "default_state": {"bottom": "true", "top": "true", "shape": "window"},
        "shapes": {"default": [[0, 0, 0, 1, 1, 1]]},
        "collision_shapes": {"default": [[0, 0, 0, 1, 1, 1]]},
    })
    write("assets/create/blockstates/" + name + ".json", {"variants": {
        "bottom=true,shape=window,top=true": {"model": "create:block/fluid_tank/block_single_window"},
        "bottom=true,shape=plain,top=true": {"model": "create:block/fluid_tank/block_single"},
    }})
    model("fluid_tank/block_single_window", [
        ([0, 0, 0], [16, 2, 16]), ([0, 14, 0], [16, 16, 16]),
        ([0, 2, 0], [2, 14, 2]), ([14, 2, 0], [16, 14, 2]),
        ([0, 2, 14], [2, 14, 16]), ([14, 2, 14], [16, 14, 16]),
    ], "minecraft:block/copper_block")
    model("fluid_tank/block_single", [([0, 0, 0], [16, 16, 16])], "minecraft:block/copper_block")
    write("assets/create/models/item/" + name + ".json", {"parent": "create:block/fluid_tank/block_single_window"})
    write("assets/create/items/" + name + ".json", {"model": {
        "type": "minecraft:model", "model": "create:item/fluid_tank"}})
    write("data/create/loot_table/blocks/" + name + ".json", {
        "type": "minecraft:block", "pools": [{"rolls": 1, "entries": [
            {"type": "minecraft:item", "name": "create:fluid_tank"}]}]})
    for locale in ["en_us", "ru_ru"]:
        path = "assets/create/lang/" + locale + ".json"
        lang = json.loads((CONTENT / path).read_text())
        lang["block.create.fluid_tank"] = "Fluid Tank"
        write(path, lang)


def saw_content():
    # Selected MIT SawBlock state/shape and one generated Java 6.0.11 recipe.
    # Fallback geometry is authored here with vanilla textures; original ARR
    # saw art is imported only from the player's pinned 6.0.10 JAR.
    directions = ["down", "up", "north", "south", "west", "east"]
    shapes = {"up": [[0, 0, 0, 1, .75, 1]], "down": [[0, .25, 0, 1, 1, 1]],
              "north": [[0, 0, .25, 1, 1, 1]], "south": [[0, 0, 0, 1, 1, .75]],
              "west": [[.25, 0, 0, 1, 1, 1]], "east": [[0, 0, 0, .75, 1, 1]]}
    write("data/create/horizon/block/mechanical_saw.json", {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 1.5, "explosion_resistance": 6,
                       "requires_tool": True, "map_color": 34,
                       "render_shape": "MODEL", "sounds": "minecraft:andesite"},
        "state_properties": {
            "facing": {"type": "enum", "values": directions},
            "axis_along_first": {"type": "boolean"},
            "flipped": {"type": "boolean"}},
        "default_state": {"facing": "up", "axis_along_first": "false", "flipped": "false"},
        "shapes": {"property": "facing", "values": shapes},
        "collision_shapes": {"property": "facing", "values": shapes},
    })
    model("mechanical_saw/vertical", [([0, 0, 0], [16, 12, 16])], "minecraft:block/andesite")
    model("mechanical_saw/horizontal", [([0, 0, 0], [16, 12, 16])], "minecraft:block/andesite")
    model("mechanical_saw/item", [([0, 0, 0], [16, 12, 16]), ([7, 12, 2], [9, 16, 14])],
          "minecraft:block/andesite")
    for part in ["active", "inactive", "reversed"]:
        model("mechanical_saw/blade_vertical_" + part,
              [([7, 10, 2], [9, 16, 14])], "minecraft:block/iron_block")
    rotations = {"up": {}, "down": {"x": 180}, "north": {"x": 90},
                 "south": {"x": 90, "y": 180}, "west": {"x": 90, "y": 270},
                 "east": {"x": 90, "y": 90}}
    variants = {}
    for facing, rotation in rotations.items():
        for first in [False, True]:
            for flipped in [False, True]:
                key = ("axis_along_first=" + str(first).lower() + ",facing=" + facing
                       + ",flipped=" + str(flipped).lower())
                turn = 90 if first and facing in ["up", "down"] else 0
                variants[key] = {"model": "create:block/mechanical_saw/" +
                                 ("vertical" if facing in ["up", "down"] else "horizontal"),
                                 **rotation}
                if turn: variants[key]["y"] = (variants[key].get("y", 0) + turn) % 360
    write("assets/create/blockstates/mechanical_saw.json", {"variants": variants})
    write("assets/create/models/item/mechanical_saw.json",
          {"parent": "create:block/mechanical_saw/item"})
    write("assets/create/items/mechanical_saw.json", {"model": {
        "type": "minecraft:model", "model": "create:item/mechanical_saw"}})
    write("data/create/loot_table/blocks/mechanical_saw.json", {"type": "minecraft:block",
        "pools": [{"rolls": 1, "entries": [{"type": "minecraft:item",
                                              "name": "create:mechanical_saw"}]}]})
    write("data/create/recipe/cutting/andesite_alloy.json", {
        "type": "create:cutting", "ingredients": [{"item": "create:andesite_alloy"}],
        "processing_time": 200, "results": [{"count": 6, "id": "create:shaft"}]})
    write("data/create/recipe/crafting/kinetics/mechanical_saw.json", {
        "type": "minecraft:crafting_shaped", "category": "misc",
        "key": {"A": "#c:plates/iron", "C": "create:andesite_casing", "I": "#c:ingots/iron"},
        "pattern": [" A ", "AIA", " C "],
        "result": {"count": 1, "id": "create:mechanical_saw"}})
    for locale, label in [("en_us", "Mechanical Saw"), ("ru_ru", "Механическая пила")]:
        path = CONTENT / ("assets/create/lang/" + locale + ".json")
        lang = json.loads(path.read_text())
        lang["block.create.mechanical_saw"] = label
        path.write_text(json.dumps(lang, ensure_ascii=False, indent=2) + "\n")


def funnel_content():
    # Selected standing ceiling shape; no Belt Funnel or original art bytes.
    shape = [[2/16, 8/16, 2/16, 14/16, 18/16, 14/16],
             [1/16, 8/16, 1/16, 15/16, 15/16, 15/16],
             [0, 6/16, 0, 1, 12/16, 1]]
    write("data/create/horizon/block/andesite_funnel.json", {
        "template": "minecraft:glass", "item": True,
        "properties": {"hardness": 1.5, "render_shape": "MODEL",
                       "sounds": "minecraft:andesite"},
        "state_properties": {
            "facing": {"type": "enum", "values": ["down"]},
            "extracting": {"type": "boolean"},
            "powered": {"type": "boolean"}},
        "default_state": {"facing": "down", "extracting": "true", "powered": "false"},
        "shapes": {"default": shape}, "collision_shapes": {"default": shape},
    })
    write("assets/create/blockstates/andesite_funnel.json", {
        "variants": {f"facing=down,extracting={extracting},powered={powered}":
                     {"model": "create:block/andesite_funnel/block"}
                     for extracting in ("false", "true") for powered in ("false", "true")}
    })
    model("andesite_funnel/block", [([2,8,2],[14,18,14]),
                                    ([1,8,1],[15,15,15]),
                                    ([0,6,0],[16,12,16])], "minecraft:block/andesite")
    write("assets/create/models/item/andesite_funnel.json",
          {"parent": "create:block/andesite_funnel/block"})
    write("assets/create/items/andesite_funnel.json", {"model": {
        "type": "minecraft:model", "model": "create:item/andesite_funnel"}})
    write("data/create/loot_table/blocks/andesite_funnel.json", {
        "type": "minecraft:block", "pools": [{"rolls": 1, "entries": [
            {"type": "minecraft:item", "name": "create:andesite_funnel"}]}]})
    for language, name in (("en_us", "Andesite Funnel"),
                           ("ru_ru", "Андезитовая воронка")):
        path = CONTENT / f"assets/create/lang/{language}.json"
        lang = json.loads(path.read_text())
        lang["block.create.andesite_funnel"] = name
        write(f"assets/create/lang/{language}.json", lang)
    path = CONTENT / "data/create/horizon/creative_tab/kinetics.json"
    tab = json.loads(path.read_text())
    from catalog import BLOCKS, ITEMS
    tab["items"] = ["create:" + name for name in BLOCKS + ITEMS]
    write("data/create/horizon/creative_tab/kinetics.json", tab)


def main():
    CONTENT.mkdir(exist_ok=True)
    (CONTENT / "horizon-mod.toml").write_text('''id = "create"
name = "Create for Horizon — prototype"
version = "0.5.0-ch4os.13"
description = "Experimental kinetic foundation with selected milling, fan, drilling, pressing and stationary saw processing, depots, crafting and a bounded one-pole mechanical piston."
authors = ["create-horizon contributors"]
api = "0.2"
side = "both"
capabilities = ["registries"]
''')
    for block in BLOCKS:
        declaration = {"template": "minecraft:glass", "item": True,
                       "properties": {"hardness": 1.5, "render_shape": "ENTITYBLOCK_ANIMATED", "sounds": "minecraft:andesite"}}
        if block in ["cogwheel", "large_cogwheel"]:
            declaration["properties"]["sounds"] = "minecraft:stripped_spruce_wood"
        if block in ["shaft", "cogwheel", "large_cogwheel"]:
            declaration["state_properties"] = {"axis": {"type": "enum", "values": ["x", "y", "z"]}}
            declaration["state_properties"]["waterlogged"] = {"type": "boolean"}
            declaration["default_state"] = {"axis": "y", "waterlogged": "false"}
            shapes = {"x": [[0, .375, .375, 1, .625, .625]],
                      "y": [[.375, 0, .375, .625, 1, .625]],
                      "z": [[.375, .375, 0, .625, .625, 1]]}
            if "cogwheel" in block:
                shapes = {"x": [[.375, 0, 0, .625, 1, 1]],
                          "y": [[0, .375, 0, 1, .625, 1]],
                          "z": [[0, 0, .375, 1, 1, .625]]}
            declaration["shapes"] = {"property": "axis", "values": shapes}
        elif block in ["creative_motor", "mechanical_press"]:
            values = ["north", "south", "east", "west"]
            if block == "creative_motor": values += ["up", "down"]
            declaration["state_properties"] = {"facing": {"type": "enum", "values": values}}
            declaration["default_state"] = {"facing": "east"}
        else:
            # Create's depot is not an inventory screen or a directional barrel.
            declaration["state_properties"] = {"waterlogged": {"type": "boolean"}}
            declaration["default_state"] = {"waterlogged": "false"}
            declaration["shapes"] = {"default": [[0, 0, 0, 1, 13 / 16, 1]]}
        write("data/create/horizon/block/" + block + ".json", declaration)
        # A blockstate model is still needed for the block item and model baking.
        name = {"creative_motor": "creative_motor/block", "mechanical_press": "mechanical_press/block", "depot": "depot/block"}.get(block, block)
        write("assets/create/blockstates/" + block + ".json", {"variants": {"": {"model": "create:block/" + name}}})
        write("assets/create/models/item/" + block + ".json", {"parent": "create:block/" + name})
        write("assets/create/items/" + block + ".json", {"model": {"type": "minecraft:model", "model": "create:item/" + block}})
        write("data/create/loot_table/blocks/" + block + ".json", {
            "type": "minecraft:block", "pools": [{"rolls": 1, "entries": [{"type": "minecraft:item", "name": "create:" + block}]}]})
    shaft = [([6, 0, 6], [10, 16, 10])]
    model("shaft", shaft)
    model("shaft_half", [([6, 6, 8], [10, 10, 16])])
    for name, low, high in [("cogwheel", 2, 14), ("large_cogwheel", -1, 17)]:
        boxes = shaft + [([low, 6, low], [high, 10, high])]
        for x, z in [(0, 6), (6, 0), (12, 6), (6, 12)]:
            boxes.append(([x, 5, z], [x + 4, 11, z + 4]))
        model(name, boxes, "minecraft:block/oak_planks")
    model("creative_motor/block", [([1, 0, 1], [15, 14, 15])], "minecraft:block/copper_block")
    model("creative_motor/block_vertical", [([1, 0, 1], [15, 14, 15])], "minecraft:block/copper_block")
    model("mechanical_press/block", [([0, 8, 0], [16, 16, 16])])
    model("mechanical_press/head", [([6, -12, 6], [10, 9, 10]), ([2, -14, 2], [14, -10, 14])])
    model("depot/block", [([1, 0, 1], [15, 12, 15]), ([0, 12, 0], [16, 13, 16])], "minecraft:block/smooth_stone")
    (CONTENT / "data/create/horizon/menu/depot.json").unlink(missing_ok=True)
    for name in SHEETS:
        write("data/create/horizon/item/" + name + "_sheet.json", {"template": "minecraft:iron_ingot", "max_stack_size": 64})
        vanilla = "gold" if name == "golden" else name
        write("assets/create/models/item/" + name + "_sheet.json", {"parent": "minecraft:item/generated", "textures": {"layer0": "minecraft:item/" + vanilla + "_ingot"}})
        write("assets/create/items/" + name + "_sheet.json", {"model": {"type": "minecraft:model", "model": "create:item/" + name + "_sheet"}})
        # Bootstrap recipes; foundation() below replaces them with the retained
        # MIT recipes, including their convention-tag ingredients.
        write("data/create/recipe/pressing/" + vanilla + "_ingot.json", {
            "type": "create:pressing", "ingredients": ["minecraft:" + vanilla + "_ingot"],
            "results": [{"id": "create:" + name + "_sheet"}],
        })
    ids = BLOCKS + [name + "_sheet" for name in SHEETS]
    write("data/create/horizon/creative_tab/kinetics.json", {"title": "itemGroup.create.base", "icon": "create:cogwheel", "items": ["create:" + name for name in ids]})
    english = ["Creative Motor", "Shaft", "Cogwheel", "Large Cogwheel", "Mechanical Press", "Depot"]
    russian = ["Творческий мотор", "Вал", "Шестерня", "Большая шестерня", "Механический пресс", "Депо"]
    for language, names, title in [("en_us", english, "Create for Horizon"), ("ru_ru", russian, "Create для Horizon")]:
        lang = {"itemGroup.create.base": title}
        lang.update({"block.create." + block: name for block, name in zip(BLOCKS, names)})
        lang.update({"item.create." + sheet + "_sheet": name for sheet, name in zip(SHEETS,
                    ["Iron Sheet", "Golden Sheet", "Copper Sheet"] if language == "en_us" else ["Железная пластина", "Золотая пластина", "Медная пластина"])})
        write("assets/create/lang/" + language + ".json", lang)
    foundation()
    mechanical_piston_content()
    portable_interface_content()
    dry_mixing_content()
    coal_heating_content()
    belt_content()
    fluid_tank_content()
    fluid_pump_content(CONTENT)
    straight_track_content()
    saw_content()


    funnel_content()
    write("data/create/recipe/mixing/dough_by_mixing.json", {'type': 'create:mixing', 'ingredients': ['#c:flours/wheat', {'type': 'neoforge:single', 'amount': 1000, 'fluid': 'minecraft:water'}], 'results': [{'id': 'create:dough'}]})
    # Source installs carry only the native bundle. Its documented data folder
    # therefore contains the complete public content, with no player-side Python.
    files = {path.relative_to(CONTENT).as_posix(): path.read_text()
             for path in sorted(CONTENT.rglob("*")) if path.is_file()
             and "target" not in path.relative_to(CONTENT).parts}
    bundled = NATIVE / "data/content.json"
    bundled.parent.mkdir(parents=True, exist_ok=True)
    bundled.write_text(json.dumps(files, indent=2, ensure_ascii=False) + "\n")
    from catalog import BLOCKS as all_blocks, ITEMS
    print(f"Generated {len(all_blocks)} blocks, {len(ITEMS)} standalone items and bundled foundation content.")


if __name__ == "__main__":
    main()
