import { Navigate } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";

function ProtectedRoute({ children }) {
  const { user, loading } = useAuth();

  
  if (loading) {
    return (
      <div>Chargement...</div>
    );
  }

  if (!user) {
    return <Navigate to="/authentication" replace />;
  }


  return children;
}

export default ProtectedRoute;