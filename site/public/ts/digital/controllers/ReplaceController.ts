import { SelectAction } from "core/actions/selection/SelectAction";
import { Component, IOObject } from "core/models";
import { GetAllPorts } from "core/utils/ComponentUtils";
import { InputPort } from "digital/models/ports/InputPort";
import { OutputPort } from "digital/models/ports/OutputPort";
import {DesignerController} from "site/shared/controllers/DesignerController";

export class ReplaceController extends DesignerController {
    public show(objs: IOObject[]): void{

    }
    public getCompatibleComponents(): IOObject[] {
        const selection = this.getSelections().filter(s => s instanceof Component) as Component[];
        const ports = GetAllPorts(selection); //selection[0].getPorts(;
        const inputs = ports.filter(o => o instanceof InputPort);
        const outputs = ports.filter(o => o instanceof OutputPort);
        return[]; 
    }
}