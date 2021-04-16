import {Camera} from "math/Camera";

import {Renderer} from "core/rendering/Renderer";

import { InputArray } from "digital/models/ioobjects/inputs/InputArray";
import { Images } from "digital/utils/Images";
import { V } from "Vector";

export const InputArrayRenderer = (() => {
    return {
        render(renderer: Renderer, component: InputArray): void {
            const size = component.getSize();

            // Get the image name 
            const image = Images.GetImage(component.isOn() ? 
                component.getOnImageName() : 
                component.getOffImageName());

            // Draw the images
            for (let i = 0; i < component.getPressableBoxes().length; i++)
            {
                renderer.image(image, V(), size);
            }
        }
    }
})();