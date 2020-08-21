import { AnalogComponent } from "./AnalogComponent";

export abstract class NetlistComponent extends AnalogComponent {

    setNetlistNumber(num: number): void {
        this.netlistNum = num;
    }

    abstract getNetlistSymbol(): string;
    abstract getNetListStats(): string;
}