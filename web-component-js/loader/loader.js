import WebComponent from "../web-component.js"

export default class Loader {
    async load(module, data, template) {
        class DefaultWebComponent extends WebComponent {}
        let name = module.manifest.name;
        let path = module.manifest.path;
        this._defineWebComponent(DefaultWebComponent, name, path, data, template)
    }

    _defineWebComponent(webComponent, name, PATH, DATA, TEMPLATE) {
        // Store path in its prototype so all its instance can access it.
        webComponent.prototype.component = {
            PATH,
            TEMPLATE,
            DATA
        };
        if (!customElements.get(name)) {
            customElements.define(name, webComponent);
        }
    }
}
