'use client'

import axios from "axios"


type apiResponse = {
	code: string,
	pk: string,
}


const getGooseContract = async (): Promise<apiResponse> => {
	try {
		const url = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";
		console.log("url", url);
		let res = await axios.get<apiResponse>(url);
		console.log("res", res.data);
		return res.data;
	} catch (err) {
		console.log("err", err);
		throw err;
	}

}



export default getGooseContract
