import {S_KEY} from "core/utils/Constants";
import {Input} from "core/utils/Input";
import {Tool} from "core/tools/Tool";

import {setSAVED} from "core/utils/Config";
import { RemoteController } from "site/shared/controllers/RemoteController";
import { SideNavController } from "site/shared/controllers/SideNavController";
import { MainDesignerController } from "site/shared/controllers/MainDesignerController";

export class SaveTool {
    private disabled: boolean;
    private sidenav: SideNavController;
    private main: MainDesignerController;

    public constructor(main: MainDesignerController, sidenav: SideNavController) {
        this.sidenav = sidenav;
        this.main = main;
        this.disabled = false;
    }

    public onEvent(_: Tool, event: string, input: Input, key?: number): boolean {
        if (this.disabled)
            return false;
        if (event != "keydown")
            return false;

        // Save: CMD/CTRL + S
        if (input.isModifierKeyDown() && key == S_KEY) {
            const data = this.main.saveCircuit();
            RemoteController.SaveCircuit(data, async () => {
                // set saved to true (which calls callbacks to set the button as invisible)
                setSAVED(true);
                return this.sidenav.updateUserCircuits();
            });
            setSAVED(false);
            return true;
        }

        return false;
    }

    public setDisabled(val: boolean = true): void {
        this.disabled = val;
    }
}
