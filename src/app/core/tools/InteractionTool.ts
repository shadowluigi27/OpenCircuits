import {Vector} from "Vector";

import {CircuitInfo} from "core/utils/CircuitInfo";
import {Event} from "core/utils/Events";
import {isPressable} from "core/utils/Pressable";

import {IOObject} from "core/models";

import {DefaultTool} from "./DefaultTool";

import {EventHandler} from "./EventHandler";
import {SelectAllHandler}     from "./handlers/SelectAllHandler";
import {FitToScreenHandler}   from "./handlers/FitToScreenHandler";
import {DuplicateHandler}     from "./handlers/DuplicateHandler";
import {DeleteHandler}        from "./handlers/DeleteHandler";
import {SnipWirePortsHandler} from "./handlers/SnipWirePortsHandler";
import {DeselectAllHandler}   from "./handlers/DeselectAllHandler";
import {SelectionHandler}     from "./handlers/SelectionHandler";
import {SelectPathHandler}    from "./handlers/SelectPathHandler";
import {UndoHandler}          from "./handlers/UndoHandler";
import {RedoHandler}          from "./handlers/RedoHandler";
import {HoverHandler} from "core/tools/handlers/HoverHandler";


export class InteractionTool extends DefaultTool {
    public constructor(handlers: EventHandler[] =
            [SelectAllHandler, FitToScreenHandler, DuplicateHandler,
             DeleteHandler, SnipWirePortsHandler, DeselectAllHandler,
             SelectionHandler, SelectPathHandler, RedoHandler, UndoHandler, HoverHandler]) {
        super(...handlers);
    }

    private findObject(pos: Vector, {designer}: Partial<CircuitInfo>): IOObject {
        // Very specifically get the objects and wires and reverse them SEPARATELY
        //  doing `designer.getAll().reverse()` would put the wires BEFORE the objects
        //  which will cause incorrect behavior! Objects are always going to need to be
        //  pressed/selected before wires!
        const objs = designer.getObjects().reverse();
        const wires = designer.getWires().reverse();
        return (objs as IOObject[]).concat(wires).find(o => (isPressable(o) && o.isWithinPressBounds(pos) ||
                                                             o.isWithinSelectBounds(pos)));
    }

    public onActivate(event: Event, info: CircuitInfo): boolean {
        return this.onEvent(event, info);
    }

    public onEvent(event: Event, info: CircuitInfo): boolean {
        const {locked, input, camera, currentlyPressedObject} = info;

        const worldMousePos = camera.getWorldPos(input.getMousePos());
        const obj = this.findObject(worldMousePos, info);

        switch (event.type) {
            case "mousedown":
                info.currentlyPressedObject = obj;

                // Check if the object is "Pressable" and
                //  if we should call their ".press" method
                if (isPressable(obj) && obj.isWithinPressBounds(worldMousePos)) {
                    obj.press();
                    return true;
                }
                break;

            case "mouseup":
                // Release currently pressed object
                if (isPressable(currentlyPressedObject)) {
                    currentlyPressedObject.release();
                    info.currentlyPressedObject = undefined;
                    return true;
                }
                info.currentlyPressedObject = undefined;
                break;

            case "click":
                // Find and click object
                if (isPressable(obj) && obj.isWithinPressBounds(worldMousePos)) {
                    obj.click();
                    return true;
                }
                break;

            case "mouseenter":
                //hover handlers here to ease debugging.
                console.log("hit");

                //find anything at this location
                info.designer.getObjects().find(o => (o.isWithinSelectBounds(worldMousePos)));
                //find type of object
                let tmp = this.findObject(worldMousePos, info);

                if (isPressable(obj) && obj.isWithinPressBounds(worldMousePos)){
                    //case: mouse is over a port, addresses I-86

                    //other cases, expand on here later

                }
                break;

        }

        if (locked)
            return false;

        return super.onEvent(event, info);
    }

}
