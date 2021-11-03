import {IO_PORT_RADIUS, LEFT_MOUSE_BUTTON} from "core/utils/Constants";

import {Event} from "core/utils/Events";
import {CircuitInfo} from "core/utils/CircuitInfo";

import {EventHandler} from "../EventHandler";
import {CreateDeselectAllAction, SelectAction} from "core/actions/selection/SelectAction";
import {GroupAction} from "core/actions/GroupAction";
import {GetAllPorts} from "core/utils/ComponentUtils";
import {Component, Port, Wire} from "core/models";
import {ShiftAction} from "core/actions/ShiftAction";
import {Circle} from "core/rendering/shapes/Circle";
import {PortInvertAction} from "core/actions/ports/PortInvertAction";
import {OutputPort} from "digital/models";


export const HoverHandlerEnter: EventHandler = ({
    conditions: (event: Event, {}: CircuitInfo) =>
        (event.type === "mousemove"),

    getResponse: ({input, camera, history, designer}: CircuitInfo) => {
        const action = new GroupAction();
        const worldMousePos = camera.getWorldPos(input.getMousePos());

        const ports = GetAllPorts(designer.getObjects());
        const objs = designer.getAll() as (Component | Wire)[];
        // Check if an object was clicked
        const obj = objs.find(o => o.isWithinSelectBounds(worldMousePos));
        const port = ports.find(o => o.isWithinSelectBounds(worldMousePos)); //need some way to detect between 2 points, but that can be left to another function
        // If we clicked a port and also hit a wire,
        //  we want to prioritize the port, so skip selecting
        if ((!(obj instanceof Wire) || port) && input.isKeyDown(666)) {

            // Select object
            if (obj) {
                console.log(obj);
            }else if (port instanceof OutputPort){
                console.log(port);
                //action.add(new PortHoverAction);
            }
        }

        // https://github.com/OpenCircuits/OpenCircuits/issues/622
        if (!action.isEmpty())
            history.add(action);
    }
});
