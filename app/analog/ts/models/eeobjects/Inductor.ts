import {serializable} from "serialeazy";

import {IO_PORT_LENGTH} from "core/utils/Constants";

import {V} from "Vector";
import {ClampedValue} from "math/ClampedValue";

import {AnalogComponent} from "analog/models/AnalogComponent";

@serializable("Inductor")
export class Inductor extends AnalogComponent {
    public constructor(inductance: number = 5) {
        super(new ClampedValue(2), V(50, 30));

        this.inductance = inductance;

        this.ports.getPorts()[0].setOriginPos(V(this.getSize().x/2, 0));
        this.ports.getPorts()[0].setTargetPos(V(IO_PORT_LENGTH, 0));

        this.ports.getPorts()[1].setOriginPos(V(-this.getSize().x/2, 0));
        this.ports.getPorts()[1].setTargetPos(V(-IO_PORT_LENGTH, 0));
    }

    public getDisplayName(): string {
        return "Inductor";
    }

    public getImageName(): string {
        return "inductor.svg";
    }

    public getInductance(): number {
        return this.inductance;
    }

    public setInductance(newInductance: number): void {
        if (newInductance > 0) {
            this.inductance = newInductance;
        }
    }
}

