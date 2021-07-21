import {S_KEY} from "core/utils/Constants";

import {Event} from "core/utils/Events";
import {CircuitInfo} from "core/utils/CircuitInfo";

import {EventHandler} from "../EventHandler";
import {AppStore} from "digital/src/state";
import {CircuitInfoHelpers} from "shared/utils/CircuitInfoHelpers";
import {SaveCircuit} from "shared/state/CircuitInfo/actions";

export function CreateSaveHandler(store: AppStore, helpers: CircuitInfoHelpers) {

    const SaveHandler: EventHandler = ({
        conditions: (event: Event, {input}: CircuitInfo) =>
            (event.type === "keydown" && event.key === S_KEY && input.isModifierKeyDown()),

        getResponse: ({}: CircuitInfo) => store.dispatch(SaveCircuit(helpers.GetSerializedCircuit()))
    });

    return SaveHandler;
}
