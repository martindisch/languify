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

    getUnclassifiedText = async () => {
        const response = await window.fetch(this.unclassifiedEndpoint, {
            method: "POST",
        });

        return response.json();
    };

    addClassifiedText = async (id: number, language: string) => {
        await window.fetch(this.classifiedEndpoint, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ id, language }),
        });
    };
}

export { LanguifyApi, UnclassifiedText };
