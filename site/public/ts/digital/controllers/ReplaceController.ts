import { SelectAction } from "core/actions/selection/SelectAction";
import { Component, IOObject } from "core/models";
import { GetAllPorts } from "core/utils/ComponentUtils";
import { InputPort } from "digital/models/ports/InputPort";
import { OutputPort } from "digital/models/ports/OutputPort";
import {DesignerController} from "site/shared/controllers/DesignerController";
import Data from "site/data/digitalnavconfig.json";
import {Create} from "serialeazy";
import { data } from "jquery";
export class ReplaceController extends DesignerController {
    public show(objs: IOObject[]): void{

    }
    public getID(item): string {
        if(item.id){
            return item.id;
        }
        else{
            return;
        }
    }
    //This returns an array that contains all the compatible components that matches in # of  I/O ports
    //from the selected array. We can use this array to display a menu that will allow that user to select
    //which component they want to replace with.
    public getCompatibleComponents(): IOObject[] {
        //Get all the components selected so we can find the total amount on I/O ports
        const selection = this.getSelections().filter(s => s instanceof Component) as Component[];
        //There are 2 types of Ports but there are no getters that returns the specific type of port,
        //So we got to filter all the ports 
        const ports = GetAllPorts(selection); //selection[0].getPorts(;
        const inputs = ports.filter(o => o instanceof InputPort);
        const outputs = ports.filter(o => o instanceof OutputPort);
        //Get the Data and then filter it so we can get the component IDs
        const DataSection = Data.sections; 
        //Within the sections, we need the items
        const componentItems = Array.from(DataSection, x => x.items);
        //Within the items, we need its ID
        const componentID = Array.from(componentItems, x => this.getID(x));
        //Using the ID, we create an array of components
        const ComponentList = Array.from(componentID, x => Create<Component>(x));
        //From the ComponentList, find all the components that matches in the amount of input Ports and
        //outputPorts
        const CompatibleComponents = ComponentList.filter(o => (o.getPorts().filter(x => x instanceof InputPort).length == inputs.length
                                                                && o.getPorts().filter(x => x instanceof OutputPort).length == outputs.length));
        return CompatibleComponents; 
    }
    protected createComponent(uuid: string): Component {
        return Create<Component>(uuid);
    }
}