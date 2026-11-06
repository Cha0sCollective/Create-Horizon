#!/usr/bin/env python3
"""Generate the selected stationary X-axis Crushing Wheels content.

Block IDs, axis, controller collision and item/loot contract follow the pinned
Create sources. The provisional model is authored here with vanilla textures;
the player's pinned Create OBJ can overlay it locally through the parts catalog.
"""

import json
from pathlib import Path


def generate(content: Path) -> None:
    def write(path: str, value: dict) -> None:
        target = content / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes((json.dumps(value, ensure_ascii=False, indent=2) + "\n").encode("utf-8"))

    write("data/create/horizon/block/crushing_wheel.json", {
        "template": "minecraft:glass", "item": True,
        "properties": {
            "hardness": 1.5, "explosion_resistance": 6,
            "requires_tool": True, "render_shape": "INVISIBLE",
            "sounds": "minecraft:andesite",
        },
        "state_properties": {"axis": {"type": "enum", "values": ["x", "y", "z"]}},
        "default_state": {"axis": "x"},
    })
    shapes = {
        "down": [[0, 0, 0, 1, .8125, 1]],
        "up": [[0, .1875, 0, 1, 1, 1]],
        "north": [[0, 0, 0, 1, 1, .8125]],
        "south": [[0, 0, .1875, 1, 1, 1]],
        "west": [[0, 0, 0, .8125, 1, 1]],
        "east": [[.1875, 0, 0, 1, 1, 1]],
    }
    write("data/create/horizon/block/crushing_wheel_controller.json", {
        "template": "minecraft:glass", "item": False,
        "properties": {
            "hardness": 0, "render_shape": "INVISIBLE",
            "sounds": "minecraft:andesite",
        },
        "state_properties": {
            "facing": {"type": "enum", "values": ["down", "up", "north", "south", "west", "east"]},
            "valid": {"type": "boolean"},
        },
        "default_state": {"facing": "down", "valid": "false"},
        "shapes": {"property": "facing", "values": shapes},
        "collision_shapes": {"property": "facing", "values": shapes},
    })
    write("assets/create/blockstates/crushing_wheel.json", {"variants": {
        "axis=x": {"model": "create:block/crushing_wheel/block", "x": 90, "y": 90},
        "axis=y": {"model": "create:block/crushing_wheel/block"},
        "axis=z": {"model": "create:block/crushing_wheel/block", "x": 90, "y": 180},
    }})
    write("assets/create/blockstates/crushing_wheel_controller.json", {
        "variants": {"": {"model": "minecraft:block/air"}},
    })
    faces = {face: {"texture": "#rim"} for face in
             ("down", "up", "north", "south", "west", "east")}
    metal_faces = {face: {"texture": "#metal"} for face in faces}
    boxes = [
        ([0, 6, 0], [16, 10, 3], faces),
        ([0, 6, 13], [16, 10, 16], faces),
        ([0, 6, 3], [3, 10, 13], faces),
        ([13, 6, 3], [16, 10, 13], faces),
        ([3, 7, 3], [13, 9, 13], metal_faces),
        ([6, 2, 6], [10, 14, 10], metal_faces),
    ]
    write("assets/create/models/block/crushing_wheel/block.json", {
        "textures": {
            "rim": "minecraft:block/andesite", "metal": "minecraft:block/iron_block",
            "particle": "minecraft:block/andesite",
        },
        "elements": [{"from": low, "to": high, "faces": sides} for low, high, sides in boxes],
    })
    write("assets/create/models/item/crushing_wheel.json", {
        "parent": "create:block/crushing_wheel/block",
    })
    write("assets/create/items/crushing_wheel.json", {"model": {
        "type": "minecraft:model", "model": "create:item/crushing_wheel",
    }})
    write("data/create/loot_table/blocks/crushing_wheel.json", {
        "type": "minecraft:block", "pools": [{"rolls": 1, "entries": [{
            "type": "minecraft:item", "name": "create:crushing_wheel",
        }]}],
    })
    write("data/create/loot_table/blocks/crushing_wheel_controller.json", {
        "type": "minecraft:block", "pools": [],
    })
    for locale, label in (("en_us", "Crushing Wheel"), ("ru_ru", "Дробящее колесо")):
        path = f"assets/create/lang/{locale}.json"
        language = json.loads((content / path).read_text(encoding="utf-8"))
        language["block.create.crushing_wheel"] = label
        write(path, language)
    path = "data/create/horizon/creative_tab/kinetics.json"
    tab = json.loads((content / path).read_text(encoding="utf-8"))
    if "create:crushing_wheel" not in tab["items"]:
        tab["items"].append("create:crushing_wheel")
    write(path, tab)
