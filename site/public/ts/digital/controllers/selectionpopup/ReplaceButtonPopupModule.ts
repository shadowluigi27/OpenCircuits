import $ from "jquery";

import {Component} from "core/models/Component";
import {ICData} from "digital/models/ioobjects/other/ICData";
import {MainDesignerController} from "../../../shared/controllers/MainDesignerController";
import {ReplaceController} from "../ReplaceController";
import {SelectionPopupModule} from "../../../shared/selectionpopup/SelectionPopupModule";

export class ReplacePopupModule extends SelectionPopupModule {
    private RController: ReplaceController;

    public constructor(circuitController: MainDesignerController, RController: ReplaceController) {
        // No wrapping div
        //I'm not sure about how the JQUERY works.
        super(circuitController, $("button#popup-Replace-button"));
        this.RController = RController;

        this.el.click(() => this.push());
    }

    public pull(): void {
        const selections = this.circuitController.getSelections();
        const componentSelections = selections.filter(o => o instanceof Component) as Component[];
        //Right now, im not sure if we are only replacing one component or many.
        //I'm assuming we can only replace one component
        if (componentSelections.length != 1) {
            this.setEnabled(false);
            return;
        }
        this.setEnabled(true);
        /*const selections = this.circuitController.getSelections();
        const componentSelections = selections.filter(o => o instanceof Component) as Component[];
        if (componentSelections.length != selections.length) {
            this.setEnabled(false);
            return;
        }

        // Check if the selections are a valid IC
        const enable = ICData.IsValid(componentSelections);

        // Enable/disable the button
        this.setEnabled(false);*/
    }

    public push(): void {
        //I'm planning for the RController to have the function getCompatibleComponents() 
        //that returns compatible components. It will show it in a menu
        this.RController.show(this.RController.getCompatibleComponents() as Component[]);
    }
}
