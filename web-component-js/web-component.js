export default class WebComponent extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({mode: 'open'});
    }

    async getData() {
        // Inefficient method for deep-cloning.
        return JSON.parse(JSON.stringify(this.component.DATA));
    }

    async getTemplate() {
        return this.component.TEMPLATE;
    }

    async onload() {}

    async connectedCallback() {
        this.data = await this.getData();
        await this.#createTemplate();
        await this.#createBindings();
        this.onload(this.shadowRoot);
    }

    async #createTemplate() {
        let content  = await this.getTemplate();
        let template = this.#createTemplateFromString(content);
        let node     = this.#createNodeFromTemplate(template);
        this.shadowRoot.appendChild(node);
    }

    // Bindings

    static ATTRIBUTE_PREFIX = "component-bind:";

    parentComponent() {
        return this.shadowRoot.host.getRootNode().host;
    }

    getValue(data, index) {
        let split = index.split(".");
        for (let i = 0; i < split.length; i++) {
            data = data[split[i]];
        }
        return data;
    }

    async #createAttributesBindings() {
        let attributes = this.shadowRoot.host.attributes;
        this.data["attributes"] = {};
        for (var i = 0; i < attributes.length; i++) {
            let attribute = attributes[i];
            this.data["attributes"][attribute.name] = attribute.value;
            if (attribute.name.indexOf(WebComponent.ATTRIBUTE_PREFIX) == 0) {
                let name = attribute.name.substring(WebComponent.ATTRIBUTE_PREFIX.length);
                let value = attribute.value;
                let parent = this.parentComponent();
                this.data[name] = this.getValue(parent.data, value);
            }
        }
    }

    #createTextBindings(node) {
        if (node.nodeName == "#text") {
            let regex = /{{\s*(\w*\.?)*\s*}}/g;
            let matches = node.nodeValue.match(regex);
            for (let index in matches) {
                let match = matches[index];
                let data = match.replace(/{{/g, "").replace(/}}/g, "").replace(/\s*/g, "");
                let value = this.getValue(this.data, data);
                node.nodeValue = node.nodeValue.replace(match, value);
            }
        }
        let childNodes = node.childNodes;
        for (let i = 0; i < childNodes.length; i++)
            this.#createTextBindings(childNodes[i]);
    }

    async #createBindings() {
        await this.#createAttributesBindings();
        this.#createTextBindings(this.shadowRoot);
    }

    // Template creation.
    #createTemplateFromString(string) {
        let innerDiv       = document.createElement("div");
        innerDiv.innerHTML = string;
        return innerDiv.firstChild;
    }

    #createNodeFromTemplate(template) {
        let node = document.importNode(template.content, true);
        return node;
    }
}