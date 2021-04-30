import {Camera} from "math/Camera";

import {Renderer} from "core/rendering/Renderer";

import { InputArray } from "digital/models/ioobjects/inputs/InputArray";
import { Images } from "digital/utils/Images";
import { V, Vector } from "Vector";

export const InputArrayRenderer = (() => {
    return {
        render(renderer: Renderer, component: InputArray, switchImgSize: Vector): void {
            const wholeComponentSize = component.getSize();
            const oneSwitchSize = wholeComponentSize.scale(V(1, 1/component.getComponentCount()));
            const offset = oneSwitchSize.y;

            // Renderer starts in the middle of the part, we need to start at the top
            renderer.translate(V(0, -offset));

            for (let i = 0; i < component.getComponentCount(); i++) {
                const image = Images.GetImage(component.isOn(i) ? 
                    component.getOnImageName() : 
                    component.getOffImageName())

                renderer.image(image, V(0, offset*i), switchImgSize);
            }
        }
    }
})();