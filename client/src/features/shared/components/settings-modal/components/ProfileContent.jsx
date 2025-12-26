import { useAuth } from '../../../../auth/AuthContext';

const ProfileContent = () => {
    const { user, loading } = useAuth();
    

    if (loading || !user) return null;

    return (
        <>
        </>
    )
}

export default ProfileContent;
