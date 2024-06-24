import WebComponent from "/web-component.js"

export default class WebStatus extends WebComponent {
    async onload(root) {
        let button = root.getElementById("button");
        let component = this;
        let data = await component.getData();
        button.onclick = function() {
            data.status = !data.status;
        };
    }
}