import Header from "../../components/layout/Header";

export default function LandingPage() {
    return (
        <div>
            <Header isAuthenticated={false} showHomeButton={false} />
        </div>
    )
}