export function CreateWebComponent(module, interface_name, name, path) {
    let exported_name       = interface_name.replaceAll("-", "_");
    let constructor_        = exported_name + "_constructor";
    let observed_attributes = exported_name + "_observed_attributes";
    class WebComponent extends HTMLElement {
        constructor() {
            super();
            this.path = path;
            this.name = name;
            this.identity = module[constructor_](this);
        }

        async connectedCallback() {
            await module.connected(this.identity);
        }

        async disconnectedCallback() {
            await module.disconnected(this.identity);
        }

        async adoptedCallback() {
            await module.adopted(this.identity);
        }

        async attributeChangedCallback(name, oldValue, newValue) {
            await module.attribute_changed(this.identity, name, oldValue, newValue);
        }

        static get observedAttributes() {
            return module[observed_attributes]();
        }
    }
    return WebComponent;
}

export async function Import(path) {
    try { return await import(path); }
    catch(e) { return null; }
}

export async function Initialize(module) {
    await module.default();
}
