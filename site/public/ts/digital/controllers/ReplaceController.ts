import { Component, IOObject } from "core/models";
import { GetAllPorts, IOObjectSet } from "core/utils/ComponentUtils";
import { InputPort } from "digital/models/ports/InputPort";
import { OutputPort } from "digital/models/ports/OutputPort";
import {DesignerController} from "site/shared/controllers/DesignerController";
import Data from "site/data/digitalnavconfig.json";
import {Create} from "serialeazy";
import { DigitalCircuitController } from "./DigitalCircuitController";
import { DigitalCircuitDesigner } from "digital/models";
import { replaceView } from "../views/replaceView";

export class ReplaceController extends DesignerController {
    protected view: replaceView;

    private mainController: DigitalCircuitController;
    private CompatibleComponents: Component[];
    
    public constructor(mainController: DigitalCircuitController){
        super(new DigitalCircuitDesigner(1, () => this.render()), new replaceView());

        this.mainController = mainController;
    }
    public show(objs: Component[]): void{
        
            this.setActive(true);
    
            // Find compatible components
            this.CompatibleComponents = objs;
    
            // Reset designer and add the internal components
            this.designer.reset();
            //this.designer.addGroup(this.CompatibleComponents);


            this.view.show();
    
            // Render
            this.render();
            this.mainController.setActive(false);
        
        /*this.setActive(true);
        this.view.show();
        this.render();
        this.mainController.setActive(false);*/
    }
    //this takes in the item and output its the item's ID. I'm not sure what the type the ID is
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