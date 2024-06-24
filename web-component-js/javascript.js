import WebComponent from "./base.js"

export default class JavaScriptWebComponent extends WebComponent {
    createObject(attributes) {
        return new this.class(attributes);
    }

    getData() {
        return this.object.data;
    }

    onLoaded(shadowRoot) {
        this.object.onLoaded(shadowRoot);
    }
}