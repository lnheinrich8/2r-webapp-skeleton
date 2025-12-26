import { useAuth } from '../../../../auth/AuthContext';

// Styling
import './profilecontent.css'

const ProfileContent = () => {
    const { user, loading } = useAuth();


    if (loading || !user) return null;

    return (
        <div className="profile-content-main">
            
        </div>
    )
}

export default ProfileContent;
