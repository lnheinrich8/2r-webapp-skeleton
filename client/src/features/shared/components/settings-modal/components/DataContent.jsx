import { useAuth } from '../../../../auth/AuthContext';

// Styling
import './smcontent.css'

const DataContent = () => {
    const { user, loading } = useAuth();


    if (loading || !user) return null;

    return (
        <div className="sm-content-main">
            data content in here
        </div>
    )
}

export default DataContent;
