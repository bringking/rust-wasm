import React, {Component} from 'react';
import ReactDOM from 'react-dom';
import { Listing, LatLng } from "./dist/rust_wasm";
import { booted } from "./dist/rust_wasm_wasm";

class App extends Component {
   constructor(props) {
    super(props);
    this.state = {
      listingOne: null,
      listingTwo: null
    }
  }

  componentDidMount() {
    booted.then(() => {
      const listingOne = Listing.new('The Downtown Dispensary', LatLng.new(32.228086, -110.96811));
      const listingTwo = Listing.new('Botanica', LatLng.new(32.31976, -111.04795));
      this.setState({listingOne, listingTwo});
    });
  }

  componentWillUnmount() {
    const {listingOne, listingTwo} = this.state;
    // Free up the Memory allocated for the Rust Structs
    listingOne.free();
    listingTwo.free();
  }

  render() {
     const {listingOne, listingTwo} = this.state;

     if(!listingOne || !listingTwo) {
       return <div>Booting WASM....</div>
     }

    return (<div>
      <h1>{listingOne.get_name()}</h1>
      <h2>{listingOne.get_location().to_json()}</h2>
      <hr/>
      <h1>{listingTwo.get_name()}</h1>
      <h2>{listingTwo.get_location().to_json()}</h2>
      <div>
        We are {listingOne.distance_to(listingTwo)} away from each other!
      </div>
    </div>);
  }
}

// Render
ReactDOM.render(<App/>, document.getElementById('app'));
