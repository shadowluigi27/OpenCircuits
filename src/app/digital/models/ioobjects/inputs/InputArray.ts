import { Pressable } from "core/utils/Pressable";
import { ClampedValue } from "math/ClampedValue";
import { V, Vector } from "Vector";
import { PressableComponent } from "../PressableComponent";

export abstract class InputArray extends PressableComponent implements Pressable {
    protected constructor(componentCount: ClampedValue, size: Vector, pSize: Vector) {
        super(new ClampedValue(), 
              new ClampedValue(3, componentCount.getValue(), 8), 
              size, 
              pSize);
    }
}