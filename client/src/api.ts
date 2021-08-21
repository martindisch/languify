interface UnclassifiedText {
    id: number;
    text: string;
}

class LanguifyApi {
    readonly unclassified_endpoint: string;
    readonly classified_endpoint: string;

    constructor(api_url: string) {
        this.unclassified_endpoint =
            api_url + "/api/v1/texts/unclassified/_next";
        this.classified_endpoint = api_url + "/api/v1/texts/classified";
    }

    async get_unclassified_text(): Promise<UnclassifiedText> {
        const response = await window.fetch(this.unclassified_endpoint, {
            method: "POST",
        });

        return response.json();
    }
}

export { LanguifyApi, UnclassifiedText };
