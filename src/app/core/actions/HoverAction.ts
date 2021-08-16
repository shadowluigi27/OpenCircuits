import {Action} from "core/actions/Action";
import {CircuitDesigner, Component, Wire} from "core/models";
import {SelectionsWrapper} from "core/utils/SelectionsWrapper";
import {Selectable} from "core/utils/Selectable";
import {GroupAction} from "core/actions/GroupAction";

export class HoverAction implements Action{
    private selections: SelectionsWrapper;
    private obj: Selectable;

    public constructor(selections: SelectionsWrapper, obj: Selectable) {

        this.selections = selections;
        this.obj = obj;
    }

    protected normalExecute(): Action {
        this.selections.select(this.obj);

        return this;
    }


    execute(): Action {
        return undefined;
    }

    undo(): Action {
        return undefined;
    }

}

export class DeselectAction extends HoverAction {
    public constructor(selections: SelectionsWrapper, obj: Selectable) {
        super(selections, obj);
    }
}