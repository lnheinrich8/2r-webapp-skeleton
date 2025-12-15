import { useState, useEffect } from 'react';
import axios from 'axios';

import API_BASE_URL from '../../shared/utils/api';

const Test = () => {

    const [testData, setTestData] = useState(null);

    const handleTestButton = async () => {
        let id = 2;
        try {
            const res = await axios.get(`${API_BASE_URL}/user/getbyid/${id}`);
            setTestData(res.data);
            console.log(res.data);
        } catch(error) {
            console.error('Request failed:', error);
        }
    }

    return (
        <div>
            <button className="test-button" onClick={handleTestButton}>Test</button>
        </div>
    )

}

export default Test;
