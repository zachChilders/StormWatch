import React, { Component } from 'react';
import snow from './snow.png';
import './App.css';


interface WeatherState {
  location: string;
  snowfall: number;
  next_snow: number;
}

interface WeatherProps {
  location: string;
}

class Weather extends Component<WeatherProps, WeatherState> {
  constructor(props: WeatherProps) {
    super(props);

    this.state = {
      location: this.props.location,
      snowfall: 0,
      next_snow: 0,
    };
  }

  componentDidMount() {
    let location = this.props.location;
    let url = "http://localhost:3030/weather/" + location;
    fetch(url)
      .then(response => response.json())
      .then(data =>
        this.setState({
          location: data.location,
          snowfall: data.total.toFixed(),
          next_snow: data.hourly[0][0]
        }));
  }

  render() {
    const { location } = this.state;
    const { snowfall } = this.state;
    const { next_snow } = this.state;

    const element = (
      <div>
        <h1>Snowfall in {location}</h1>
        <h2>{snowfall} inches over 48 hours</h2>
        <h2>Starting in {next_snow} hours</h2>
      </div>
    );
    return element;
  }
}

interface ButtonProps {
  location: string;
  handler: any;
}

class LocationButton extends Component<ButtonProps> {
  constructor(props: any) {
    super(props);

    this.state = {
      location: this.props.location,
    };
  }

  render() {
    return (
      <button className="Button" onClick={() => this.props.handler(this.props.location)}>
        {this.props.location}
      </button>
    );
  }
}

interface ParentState {
  current_location: string;
  locations: string[];
}

class WeatherParent extends Component<{}, ParentState> {
  constructor(props: any) {
    super(props);

    this.state = {
      current_location: "Mammoth",
      locations: ["Reno", "Mammoth", "SouthLake", "Chair2", "Chair4"]
    };

    this.setLocation = this.setLocation.bind(this);
  }

  setLocation(location: string) {
    this.setState({
      current_location: location
    });
  }

  buildChildren() {
    const buttons = (
      this.state.locations.map((location, index) => {
        return (<LocationButton key={index} location={location} handler={this.setLocation} />)
      })
    );

    return (
      <div className="Weather">
        <header className="App-header">
          <img src={snow} className="App-logo" alt="logo" />
          <Weather key={this.state.current_location} location={this.state.current_location} />
          <div>{buttons}</div>
        </header>
      </div>
    );
  }

  render() {
    return <div>{this.buildChildren()}</div>
  }
}

function App() {
  return (
    <WeatherParent></WeatherParent>
  );
}

export default App;
