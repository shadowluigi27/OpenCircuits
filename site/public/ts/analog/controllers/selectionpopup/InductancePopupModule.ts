import $ from "jquery";

import {GroupAction} from "core/actions/GroupAction";
import {InductanceChangeAction} from "analog/actions/InductanceChangeAction";

import {MainDesignerController} from "../../../shared/controllers/MainDesignerController";

import {Inductor} from "analog/models/eeobjects/Inductor";

import {SelectionPopupModule} from "../../../shared/selectionpopup/SelectionPopupModule";

export class InductancePopupModule extends SelectionPopupModule {
    private input: HTMLInputElement;

    public constructor(circuitController: MainDesignerController) {
        super(circuitController, $("div#popup-inductance-text"));

        this.input = this.el.find("input#popup-inductance")[0] as HTMLInputElement;
        this.input.onchange = () => this.push();
    }

    public pull(): void {
        const selections = this.circuitController.getSelections();
        const clocks = selections
                .filter(o => o instanceof Inductor)
                .map(o => o as Inductor);

        // Only enable if there's exactly 1 type, so just inductors
        const enable = selections.length > 0 && (selections.length == clocks.length);

        if (enable) {
            // Calculate input counts for each component
            const counts: number[] = [];
            clocks.forEach(i => counts.push(i.getInductance()));

            const same = counts.every((count) => count === counts[0]);

            this.input.value = same ? counts[0].toString() : "";
            this.input.placeholder = same ? "" : "-";
        }

        this.setEnabled(enable);
    }

    public push(): void {
        const selections = this.circuitController.getSelections() as Inductor[];
        const countAsNumber = this.input.valueAsNumber;

        this.circuitController.addAction(new GroupAction(
            selections.map(i => new InductanceChangeAction(i, countAsNumber))
        ).execute());

        this.circuitController.render();
    }
}
