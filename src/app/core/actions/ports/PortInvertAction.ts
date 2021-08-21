import {Selectable} from "core/utils/Selectable";
import {Action} from "core/actions/Action";
import {Vector} from "Vector";
import {Circle} from "core/rendering/shapes/Circle";


export class PortInvertAction implements Action {
    private obj: Selectable;

    public constructor(v: Vector) {
    }

    public execute(): Action {
        //draw(new Circle(tmp, IO_PORT_RADIUS/3), circleStyle)
        return this;
    }

    public undo(): Action {

        return this;
    }

}
