import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';

// import { hybridEncrypt, loadPublicKeyFromPemUrl } from '../utils/EncryptPayload';
import API_BASE_URL from '../shared/utils/api';

// Styling
import './register.css';

const Register = () => {
    const [formData, setFormData] = useState({
        firstname: '',
        lastname: '',
        email: '',
        password: '',
        confirmPassword: '',
    });

    const navigate = useNavigate();

    const [error, setError] = useState('');

    // const [publicKey, setPublicKey] = useState(null);

    const handleChange = (e) => {
        const { name, value } = e.target;
        setFormData(prev => ({ ...prev, [name]: value }));
    };

    const handleRegister = async (e) => {
        e.preventDefault();
        if (formData.password !== formData.confirmPassword) {
            setError("Passwords do not match");
            return;
        }

        try {
            // const key = publicKey || await loadPublicKeyFromPemUrl('/keys/public_key.pem');
            // setPublicKey(key);
            const { confirmPassword, ...formDataToSend } = formData; // exclude confirmPassword
            // const encryptedPayload = await hybridEncrypt(key, {
            //     ...formDataToSend
            // });
            
            await axios.post(
                `${API_BASE_URL}/auth/register`,
                { ...formDataToSend }
            );
            navigate('/', { state: { registrationSuccess: true } });
        } catch (err) {
            const serverError = err.response?.data?.error || "Unknown error occurred during registration";
            setError(`Registration failed: ${serverError}`);
        }
    };

    const handleLoginRedirect = (e) => {
        e.preventDefault();
        navigate('/');
    };

    return (
        <div className="register-page">
            <form onSubmit={handleRegister} className="register-form">
                <h2>Create Account</h2>
                {error && <p className="error">{error}</p>}

                <input type="text" name="firstname" placeholder="First Name" value={formData.firstname} className="general-input-1" onChange={handleChange} required />
                <input type="text" name="lastname" placeholder="Last Name" value={formData.lastname} className="general-input-1" onChange={handleChange} required />
                <input type="email" name="email" placeholder="Email" value={formData.email} className="general-input-1" onChange={handleChange} required />
                <input type="password" name="password" placeholder="Password" value={formData.password} className="general-input-1" onChange={handleChange} required />
                <input type="password" name="confirmPassword" placeholder="Retype Password" value={formData.confirmPassword} className="general-input-1" onChange={handleChange} required />

                <button type="submit" className="general-button-1">Register</button>

                <p className="login-text">
                    Already have an account?{' '}
                    <span className="login-link" onClick={handleLoginRedirect}>
                        Login
                    </span>
                </p>
            </form>
        </div>
    );
};

export default Register;
