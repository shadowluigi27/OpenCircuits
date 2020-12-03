import {DigitalCircuitView} from "./DigitalCircuitView";
import $ from "jquery";

export class replaceView extends DigitalCircuitView {
    private div: HTMLDivElement;
    private closeButton: HTMLButtonElement;
    //Confirm finalizes the component to replace with
    private confirmButton: HTMLButtonElement;
    public constructor() {
        //declare canvas. Need to create an html file for this as well
        const canvas;
        if (!(canvas instanceof HTMLCanvasElement))
            throw new Error("ReplaceViewer Canvas element not found!");
        //call the super function
        super(canvas, .84,.076);
        //Need to get the HTML element(the div and the closeButton and)

    }
    public setCursor(cursor: string): void {
        this.renderer.setCursor(cursor);
    }
    public show(): void {
        this.div.classList.remove("invisible");
    }
    public hide(): void {
        this.div.classList.add("invisible");
    }
}