import { Pressable } from "core/utils/Pressable";
import { ClampedValue } from "math/ClampedValue";
import { V, Vector } from "Vector";
import { PressableComponent } from "../PressableComponent";

export abstract class InputArray extends PressableComponent implements Pressable {

    protected componentCount: ClampedValue;

    protected constructor(componentCount: ClampedValue, size: Vector, pSize: Vector) {
        super(new ClampedValue(), 
              componentCount, 
              size, 
              pSize,
              componentCount.getValue());
        this.componentCount = componentCount;
    }

    public getComponentCount(): number {
        return this.componentCount.getValue();
    }

    public click(): void {
        if (this.lastPressedBoxIndex != null) {
            this.activate(!this.on[this.lastPressedBoxIndex], this.lastPressedBoxIndex);
        } 
    }
}