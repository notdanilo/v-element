import Loader from "./loader.js"

export default class JavaScriptLoader extends Loader {
    load(module, data, template) {
        let name = module.manifest.name;
        let path = module.manifest.path;
        this._defineWebComponent(module.instance.default, name, path, data, template)
    }
}