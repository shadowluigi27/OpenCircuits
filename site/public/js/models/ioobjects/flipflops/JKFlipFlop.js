class JKFlipFlop extends Gate {
    constructor(context, x, y) {
        super(context, false, x, y, undefined);
        this.noChange = true;
        this.setInputAmount(3);
        this.setOutputAmount(2);
        this.transform.setSize(this.transform.size.scale(1.5));
		this.clock = false;
		this.last_clock = false;
    }
    onTransformChange() {
        this.transform.setSize(V(DEFAULT_SIZE, DEFAULT_SIZE));
        super.onTransformChange();
        this.transform.setSize(V(DEFAULT_SIZE*1.5, DEFAULT_SIZE*1.5));
    }
    activate(x) {
        var on = this.outputs[0].isOn;

        var set = this.inputs[0].isOn;
        var clock = this.inputs[1].isOn;
        var reset = this.inputs[2].isOn;
        if (clock && !last_clock) {
            if (set && reset) {
                on = !on;
            } else if (set) {
                on = true;
            } else if (reset) {
                on = false;
            }
        }

        super.activate(on, 0);
        super.activate(!on, 1);
		
		var last_clock = clock;
    }
    draw() {
        super.draw();

        var renderer = this.context.getRenderer();
        this.localSpace();
        renderer.rect(0, 0, this.transform.size.x, this.transform.size.y, this.getCol(), this.getBorderColor());
        renderer.restore();
    }
    getDisplayName() {
        return "JK Flip Flop";
    }
}
JKFlipFlop.getXMLName = function() { return "srff"; }
Importer.types.push(JKFlipFlop);
