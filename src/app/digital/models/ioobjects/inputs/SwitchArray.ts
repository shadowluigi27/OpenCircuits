import { ClampedValue } from "math/ClampedValue";
import { serializable } from "serialeazy";
import { V } from "Vector";
import { InputArray } from './InputArray';

@serializable("SwitchArray")
export class SwitchArray extends InputArray {
    public constructor() {
        super(new ClampedValue(3, 2, 8), 
              V(62, 77), 
              V(48, 60));
    }

    public getOffImageName(): string {
        return "switchUp.svg";
    }

    public getOnImageName(): string {
        return "switchDown.svg";
    }
    
    public getDisplayName(): string {
        return "Switch Array";
    }
}