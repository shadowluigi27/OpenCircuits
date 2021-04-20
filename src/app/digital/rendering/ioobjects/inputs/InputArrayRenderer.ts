import {Camera} from "math/Camera";

import {Renderer} from "core/rendering/Renderer";

import { InputArray } from "digital/models/ioobjects/inputs/InputArray";
import { Images } from "digital/utils/Images";
import { V } from "Vector";

export const InputArrayRenderer = (() => {
    return {
        render(renderer: Renderer, component: InputArray): void {
            const size = component.getSize();
            const offset = size.y;

            for (let i = 0; i < component.getComponentCount(); i++) {
                const image = Images.GetImage(component.isOn(i) ? 
                    component.getOnImageName() : 
                    component.getOffImageName())

                renderer.image(image, V(0, offset*i), size);
            }
        }
    }
})();