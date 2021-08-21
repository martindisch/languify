interface UnclassifiedText {
    id: number;
    text: string;
}

class LanguifyApi {
    readonly unclassifiedEndpoint: string;
    readonly classifiedEndpoint: string;

    constructor(api_url: string) {
        this.unclassifiedEndpoint =
            api_url + "/api/v1/texts/unclassified/_next";
        this.classifiedEndpoint = api_url + "/api/v1/texts/classified";
    }

    async getUnclassifiedText(): Promise<UnclassifiedText> {
        const response = await window.fetch(this.unclassifiedEndpoint, {
            method: "POST",
        });

        return response.json();
    }
}

export { LanguifyApi, UnclassifiedText };
