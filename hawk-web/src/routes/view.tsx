import { Nav } from '../components/layouts/Nav';
import '../main.css';

export const View = () => {
    document.title = 'View';
    return (
        <div className="main">
            <Nav />
        </div>
    );
};
