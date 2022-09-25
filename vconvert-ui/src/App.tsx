import { Route, Routes } from "@solidjs/router";
import Nav from "./components/nav";
import Info from "./pages/info";
import ss from "./pages/ss";

function App() {

  return (
    <div class="">
      <Nav/>
      <div class="px-5 pb-2" >

        <Routes>
          <Route path="/" component={Info} />
          <Route path="/soundspace" component={ss} />
        </Routes>
      </div>
    </div>
  );
}

export default App;
