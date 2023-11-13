import axios from "axios"

type apiResponse = {
	code: string,
	pk: string,
}


const getGooseContract = async (): Promise<apiResponse> => {
	let url = process.env.API_URL || "http://localhost:8080";
	let res = await axios.get<apiResponse>(url);

	return res.data;
}



export default getGooseContract
