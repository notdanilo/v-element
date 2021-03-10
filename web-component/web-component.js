export default class WebComponent extends HTMLElement {
    constructor() {
        super();
        // Inefficient method for deep-cloning.
        this.data = JSON.parse(JSON.stringify(this.component.DATA));
        this.attachShadow({mode: 'open'});
    }

    async getData() {
        return this.data;
    }

    async getTemplate() {
        return this.component.TEMPLATE;
    }

    async onload() {}

    async connectedCallback() {
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

    async #createBindings() {
        let data = await this.getData();
        let attributes = this.shadowRoot.host.attributes;
        data["attributes"] = {};
        for (var i = 0; i < attributes.length; i++) {
            let attribute = attributes[i];
            data["attributes"][attribute.name] = attribute.value;
        }
        let el = this.shadowRoot.getElementById("vue");

        // We need to remove style elements from the template because Vue doesn't
        // compile it.
        let styles = [];
        let styles_in_el = el.getElementsByTagName("style");
        for (var i = 0; i < styles_in_el.length; i++) {
            styles.push(styles_in_el[i].parentNode.removeChild(styles_in_el[i]))
        }

        // Vue is also not working with standard <slot> tags when a new Vue instance is created.
        // We can replace it with a <div> placeholder and recover the original slots after the
        // instance is created.
        let slots = [];
        let slots_in_el = el.getElementsByTagName("slot");
        for (var i = 0; i < slots_in_el.length; i++) {
            let elem = slots_in_el[i];
            let id   = "placeholder#" + i;
            let placeholder = document.createElement("div");
            placeholder.id = id;
            elem.replaceWith(placeholder);
            slots.push({elem,id})
        }

        Vue.config.silent = true;

        // Watch changes.
        let watch = {};
        this.vue = new Vue({el,data,watch});
        this.vue.web_component = this;

        // We then put the style elements back.
        for (var i = 0; i < styles.length; i++) {
            this.vue.$el.prepend(styles[i]);
        }

        // We can now get the slots back.
        for (var i = 0; i < slots.length; i++) {
            let slot = slots[i];
            this.shadowRoot.getElementById(slot.id).replaceWith(slot.elem)
        }
    }

    // Template creation.
    #createTemplateFromString(string) {
        let innerDiv       = document.createElement("div");
        innerDiv.innerHTML = string;
        return innerDiv.firstChild;
    }

    #createNodeFromTemplate(template) {
        let node = document.importNode(template.content, true);
        let div  = document.createElement("div");
        div.setAttribute("id", "vue");
        div.appendChild(node);
        return div;
    }
}