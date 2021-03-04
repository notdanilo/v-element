import WebComponent from "./web-component.js"

export default class WASMWebComponent extends WebComponent {
    constructor() {
        super();
        this.ffi_create       = this.#getMethod("create");
        this.ffi_template     = this.#getMethod("template");
        this.ffi_get_data     = this.#getMethod("get_data");
        this.ffi_update_field = this.#getMethod("update_field");
        this.ffi_on_loaded    = this.#getMethod("on_loaded");
    }

    #getMethod(name) {
        let module = this.module.instance;
        let method = `${this.path}_${name}`;
            method = module[method];
        return method;
    }

    async template() {
        return this.ffi_template();
    }

    createObject() {
        return this.ffi_create(this.shadowRoot.host.attributes);
    }

    getData() {
        let json = "{}";
        if (this.ffi_get_data) json = this.ffi_get_data(this.object);
        return JSON.parse(json);
    }

    onload() {
        return this.ffi_on_loaded(this.object, this.shadowRoot);
    }

    updateField(name, value) {
        this.ffi_update_field(this.object, name, value);
    }

    async connectedCallback() {
        this.object = this.createObject(this.shadowRoot.host.attributes);

//        for (var name in data) {
//            watch[name] = (function(name) {
//                return function(new_val,_) {
//                    let data = JSON.stringify(new_val);
//                    this.web_component.updateField(name, data);
//                }
//            })(name)
//        }

        super.connectedCallback();
    }
}