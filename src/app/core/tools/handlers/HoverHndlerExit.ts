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
import {HoverAction} from "core/actions/HoverAction";
import {PortInvertAction} from "core/actions/ports/PortInvertAction";


export const HoverHandlerLeave: EventHandler = ({
    conditions: (event: Event, {}: CircuitInfo) =>
        (event.type === "mouseleave"),

    getResponse: ({input, camera, history, designer, selections}: CircuitInfo) => {

    }
});
