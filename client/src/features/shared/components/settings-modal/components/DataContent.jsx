import { useAuth } from '../../../../auth/AuthContext';

// Styling
import './smcontent.css'

const DataContent = () => {
    const { user, loading } = useAuth();


    if (loading || !user) return null;

    return (
        <div className="sm-content-main">
            <div className="sm-content-row">
                <span className="sm-content-label">Placeholder</span>
                <span className="sm-content-value">Placeholder</span>
            </div>
        </div>
    )
}

export default DataContent;
