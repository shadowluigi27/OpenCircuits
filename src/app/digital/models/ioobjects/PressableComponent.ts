import {serialize} from "serialeazy";

import {Vector,V}     from "Vector";
import {Transform}    from "math/Transform";
import {RectContains} from "math/MathUtils";
import {ClampedValue} from "math/ClampedValue";

import {Pressable} from "core/utils/Pressable";

import {DigitalComponent} from "../DigitalComponent";


export abstract class PressableComponent extends DigitalComponent implements Pressable {
    @serialize
    protected pressableBoxes: Transform[];

    @serialize
    protected on: boolean[];

    @serialize
    protected lastPressedBoxIndex: number = -1;

    @serialize
    protected componentSize: Vector;

    protected constructor(inputPortCount: ClampedValue, outputPortCount: ClampedValue, size: Vector, 
                          pSize: Vector, componentCount: number = 1) {
        super(inputPortCount, outputPortCount, size);

        this.pressableBoxes = [];
        this.componentSize = size.scale(V(1, 1/componentCount));
        let adjustment = componentCount > 1 ? this.componentSize.y : 0;
        for (let i = 0; i < componentCount; i++) {
            let newBox = new Transform(V(0, this.componentSize.y*i-adjustment), pSize);
            newBox.setParent(this.transform);
            this.pressableBoxes.push(newBox);
        }

        this.on = [];
        for (let i = 0; i < componentCount; i++) {
            this.on.push(false);
        }
    }

    public activate(signal: boolean, i: number = 0): void {
        this.on[i] = signal;

        super.activate(signal, i);
    }

    public press(): void {
        console.log("aaa");
    }

    public click(): void {
        console.log("bbb");
    }

    public release(): void {
        console.log("ccc");
    }

    /**
     * Determines whether or not a point is within
     *  this component's "pressable" bounds
     * @param  v The point
     * @return   True if the point is within this component,
     *           false otherwise
     */
    public isWithinPressBounds(v: Vector): boolean {
        var i:number;
        for (i = 0; i < this.pressableBoxes.length; i++)
        {
            var box = this.pressableBoxes[i];
            if (RectContains(box, v)) {
                this.lastPressedBoxIndex = i;
                return true;
            }
        }
        this.lastPressedBoxIndex = -1;
        return false;
    }

    public isWithinSelectBounds(v: Vector): boolean {
        // Only true if we're normally in bounds and also not in the press bounds
        //   i.e. prevents selecting when pressing the button part of the Button
        return super.isWithinSelectBounds(v) && !this.isWithinPressBounds(v);
    }

    public getPressableBoxes(): Transform[] {
        return this.pressableBoxes;
    }

    public getComponentSize(): Vector {
        return this.componentSize;
    }

    public isOn(i: number = 0): boolean {
        return this.on[i];
    }

    public getMinPos(): Vector {
        const min = super.getMinPos();

        // Find minimum pos from corners of selection box
        const corners: Vector[] = [];
        for (const pressableBox of this.pressableBoxes)
            corners.concat(pressableBox.getCorners().map((v) =>
                v.sub(this.getOffset())
            ));

        return Vector.min(min, ...corners);
    }

    public getMaxPos(): Vector {
        const max = super.getMaxPos();
        
        // Find maximum pos from corners of selection box
        const corners: Vector[] = [];
        for (const pressableBox of this.pressableBoxes)
            corners.concat(pressableBox.getCorners().map((v) =>
                v.add(this.getOffset())
            ));

        return Vector.max(max, ...corners);
    }

    public getImageName(): string {
        return (this.isOn() ? this.getOnImageName() : this.getOffImageName());
    }

    public abstract getOffImageName(): string;
    public abstract getOnImageName(): string;
}
