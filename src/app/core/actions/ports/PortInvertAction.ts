import {Action} from "core/actions/Action";

import {GetPath} from "core/utils/ComponentUtils";

import {CircuitDesigner} from "core/models/CircuitDesigner";
import {Component} from "core/models/Component";
import {Port} from "core/models/ports/Port";

import {GroupAction} from "../GroupAction";
import {CreateDeletePathAction} from "../deletion/DeletePathActionFactory";

export abstract class PortInvertAction implements Action{
    protected designer: CircuitDesigner;

    protected targetCount: number;
    protected initialCount: number;

    private wireDeletionAction: GroupAction;

    protected constructor(designer: CircuitDesigner, target: number, initialCount: number) {
        this.designer = designer;

        this.targetCount = target;
        this.initialCount = initialCount;
    }


    execute(): Action {
        return undefined;
    }

    undo(): Action {
        return undefined;
    }

}