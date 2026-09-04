require("items")
require("companies")
require("leisure")
require("colors")
require("roadvehicles")
require("rollingstock")

data:extend {
    {
        type = "freight-station",
        name = "freight-station",
        label = "Freight Station",
        asset = "rail_freight_station.glb",
        price = 1000,
        size = {160, 200},
        -- Station-owned road fleet (sov-2uv, $VEHICLE_STATION precedent):
        -- imports run on these instead of borrowing factory trucks.
        n_trucks = 2,
    }
}