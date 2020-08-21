import {Action} from "core/actions/Action";
import {Inductor} from "analog/models/eeobjects/Inductor";

export class InductanceChangeAction implements Action {
    private inductor: Inductor;

    private initialInductance: number;
    private targetInductance: number;

    public constructor(inductor: Inductor, targetInductance: number) {
        this.inductor = inductor;

        this.initialInductance = inductor.getInductance();
        this.targetInductance = targetInductance;
    }

    public execute(): Action {
        this.inductor.setInductance(this.targetInductance);

        return this;
    }

    public undo(): Action {
        this.inductor.setInductance(this.initialInductance);

        return this;
    }

}